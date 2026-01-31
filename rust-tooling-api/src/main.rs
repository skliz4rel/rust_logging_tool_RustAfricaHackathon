use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer,http::header, Responder, get};

//You must register all your modules for it to be visible within your project
mod routes;

use actix_cors::Cors;
use dal_layer::models::details::Details;

use crate::routes::{health_check::*, log_routes::*, myservice_routes::*};
use dal_layer::models::{log_model::*, my_service_model::*, response_model::*};
use dal_layer::repository::db::Database;

use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::{SwaggerUi, Config};

use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
       health_check,
        create_service,
      get_services,
        create_log,
        get_logs_byservices,
   get_logs_services_by_date_range,
     
    ),
    components(
        schemas(
            LogRequest,
            GenericResponse<String>,
            MyServiceView,         
        )
    ),
    tags(
     (name = "Health", description = "Health endpoints"),
        (name = "Register Service", description = "Register A Micro Service or Monolith Endpoint"),
		(name = "Get Service", description = "Get MicroServices or Monoliths registered in the Logging system Endpoint"),
		(name = "Create Logs", description = "Create Logs for a registered Microservice Endpoint"),
		(name = "Get Logs", description = "Get Logs for a MIcro service Endpoint"),
		//(name = "Get_logs_Service", description = "Get Logs by service Endpoint"),
		(name = "Get logs by Service by date", description = "Get logs by service and date Endpoint")
    )
)]
pub struct ApiDoc;



#[utoipa::path(
	get,
	path = "/api/healthchecker",
	tag = "Health",
	responses(
		(status=200, description = "This shows the microservice is up and running", body = Details),
	)
)]
#[get("/")]
async fn hello() -> impl Responder {
    let d: Details = Details {
        name: "jide".to_string(),
        age: 3,
    };

    HttpResponse::Ok().body("Hello YouTube!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
            let cors = Cors::default()
            .allowed_origin("http://localhost:5000")
          //  .allowed_origin("https://localhost:5000")
            .allowed_methods(vec!["GET", "POST","PUT","PATCH","DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        //setting up the service and the endpoints
        App::new()
            .app_data(db_data.clone()) //register or inject the database obj
             .wrap(cors)
            .service(hello)
            .service(health_check)
    .service(
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-doc/openapi.json", ApiDoc::openapi())
         .config(
            Config::default() 
            .display_operation_id(true)     // 👈 KEEP TAG ORDER AS DEFINED
        )        
      )
            .service(create_service)
            .service(get_services)
            .service(create_log)
            .service(get_logs_byservices)
            .service(get_logs_services_by_date_range)
           
    })
    .bind(("localhost", 5000))?
    .run()
    .await
}
