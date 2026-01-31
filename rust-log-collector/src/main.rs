use chrono::Utc;
use dal_layer::models::my_service_model::MyService;
use dal_layer::models::my_service_model::MyServiceView;
use dal_layer::repository::db::Database;
use rust_log_collector::{ALogFile, Config, Directory};
use std::sync::mpsc;
use std::thread;
use tokio;

use tokio::signal;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() {
    ///Load the services stored in the database services from db
    let services: Vec<MyServiceView> = getservices().await.unwrap();

    /// Load config from json
    let filename: String = String::from("config.json");

    let mut configs: Vec<Config> = rust_log_collector::read_config(filename).unwrap();

    ///Pass the service id to each service in config and add the service that dont exits in db
    let list: &Vec<Config> = tied_service_id_2_configs(&services, &mut configs).await;

    ///read tru config (each represent a micro service and the log file directory)
    ///
    /// read thru the files
    let dir_count: usize = list.len();

    println!("got here dir count {dir_count}");

    let mut ticker = interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if dir_count > 0 {
                    println!("Reading Logs now from {:?}", list);
                    multiple_transmitter_receiver(dir_count, &list).await;
                }
            }
            _ = signal::ctrl_c() => {
                println!("Shutting down log worker...");
                break;
            }
        }
    }
}

pub async fn multiple_transmitter_receiver(count: usize, list: &Vec<Config>) {
    let (tx, rx) = mpsc::channel();

    for i in 0..count {
        // Clone the specific Config so the thread owns it
        let config = list[i].clone();
        let producer = tx.clone();

        thread::spawn(move || match config.service_id {
            Some(id) => {
                let mut dir = Directory {
                    service_id: Some(id),
                    application_name: config.application_name.clone(),
                    files: Vec::new(),
                };

                dir.read_dir(&config.application_name, &config.log_location);

                println!("printing the files in the dir {:?}", dir.files);

                producer.send(dir).unwrap(); //This is going to send a directory with list of files in it.
            }
            _ => {}
        });
    }

    drop(tx); // Close the original sender so rx will end after all clones drop

    //reciever thread
    for dir in rx {
        read_files_store_in_db(&dir).await;
        dir.delete_files_in_dir().await;
    }
}

async fn read_files_store_in_db(dir: &Directory) {
    let mut store: ALogFile = ALogFile {
        application_name: dir.application_name.to_string(),
        logs_in_file: Vec::new(),
    };

    println!("Directory content {}", dir);
    println!("number of files in dir {}", &dir.files.len());

    for path in &dir.files {
        let id: &str = dir.service_id.as_ref().unwrap();

        store.read_file_logs(&id, &path);
    }

    //store the values in the database
    store.store_in_db().await;
}

///This get the list of services
async fn getservices() -> Result<Vec<MyServiceView>, Box<dyn std::error::Error>> {
    //Result<Vec<MyServiceView>, Box<String>> {
    let db = Database::init().await;

    let list = db.get_services().await?;
    // .map_err(|e| Box::new(Error::Database(e.to_string())) as Box<dyn Error>)?;

    Ok(MyServiceView::from_bulk(list)?)
}

///This is going to connect the service id to each service in the config
async fn tied_service_id_2_configs<'a>(
    services: &'a Vec<MyServiceView>,
    configs: &'a mut Vec<Config>,
) -> &'a Vec<Config> {
    //Result<Vec<MyServiceView>, Box<String>> {
    let db = Database::init().await;

    for c in configs.iter_mut() {
        if let Some(service) = services.iter().find(|s| s.name == c.application_name) {
            c.service_id = service.service_id.clone(); // or *service.id if Copy
        } else {
            //save the service and extract the id

            let model: MyServiceView = MyServiceView {
                service_id: None,
                name: c.application_name.clone(),
                description: Some(String::from("Micro service applciation")),
                onboarded_datetime: Some(Utc::now().to_rfc3339()),
            };

            match db
                .create_service(
                    MyService::try_from(model)
                        .expect("Error converting Service request to Serviice entity."),
                )
                .await
            {
                Ok(service) => {
                    println!("verify the content {:?}", service);

                    match service.inserted_id.as_str() {
                        Some(id) => {
                            c.service_id = Some(id.to_string());
                        }
                        None => {}
                    }
                }
                Err(err) => println!("{}", err.to_string()),
            };
        }
    }

    configs
}
