use crate::models::{log_model::Log, log_model::LogLevel, my_service_model::MyService};
use crate::utils::date_helper::Converter;

use actix_web::Error;
use chrono::Utc;
use futures::stream::TryStreamExt;

use mongodb::Collection;
use mongodb::bson::from_document;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, doc};
use mongodb::options::CountOptions;
use mongodb::results::InsertOneResult;
use mongodb::results::UpdateResult;
use mongodb::{Client, Cursor};
use mongodb::{IndexModel, options::IndexOptions};
use std::env;
use std::str::FromStr;

use mongodb::options::ClientOptions;

pub struct Database {
    log: Collection<Log>,
    myservice: Collection<MyService>,
}

impl Database {
    ///This is going to initialize the database
    pub async fn init() -> Self {
        let uri = env::var("MONGO_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/?directConnection=true".into());

        // Parse options so we can tune pool settings
        let mut client_options = ClientOptions::parse(&uri)
            .await
            .expect("Failed to parse MongoDB URI");

        client_options.app_name = Some("rust-log-monitor".to_string());

        // 👇 POOL SETTINGS (optional but recommended)
        client_options.max_pool_size = Some(20); // max concurrent connections
        client_options.min_pool_size = Some(5); // keep some warm
        client_options.max_idle_time = Some(std::time::Duration::from_secs(60));

        let client =
            Client::with_options(client_options).expect("Failed to initialize MongoDB client");

        let db = client.database("rust_log_monitor");

        let myservice: Collection<MyService> = db.collection("myservice");
        let log: Collection<Log> = db.collection("log");

        Database { log, myservice }
    }

    pub async fn ensure_created_at_index(
        collection: &mongodb::Collection<Log>,
    ) -> Result<(), Error> {
        let index = IndexModel::builder()
            .keys(doc! { "created_at": 1 }) // ascending
            .options(IndexOptions::builder().build())
            .build();

        collection
            .create_index(index)
            .await
            .ok()
            .expect("Error creating the index");

        Ok(())
    }

    //insert my service into the database
    pub async fn create_service(&self, myservice: MyService) -> Result<InsertOneResult, Error> {
        let result = self
            .myservice
            .insert_one(myservice)
            .await
            .ok()
            .expect("Error inserting application or microserice");

        Ok(result)
    }

    /// Bulk insert using a single `insert_many` call
    pub async fn insert_services_bulk(&self, services: Vec<MyService>) -> Result<(), Error> {
        if services.is_empty() {
            return Ok(()); // nothing to insert
        }

        // Ordered=false means MongoDB will continue inserting even if one fails (e.g., duplicate key)
        let options = mongodb::options::InsertManyOptions::builder()
            .ordered(false)
            .build();

        self.myservice
            .insert_many(services)
            .await
            .ok()
            .expect("problem inserting bulk service");

        Ok(())
    }

    pub async fn get_services(&self) -> Result<Vec<MyService>, Error> {
        // No filter = return all documents (be careful with large collections!)

        let filter = doc! {};

        let mut cursor: Cursor<MyService> = self
            .myservice
            .find(filter)
            .await
            .ok()
            .expect("error reading through the services");

        let services: Vec<MyService> = cursor
            .try_collect()
            .await
            .ok()
            .expect("error displaying the list of services"); // or iterate with try_next

        Ok(services)
    }

    /******************************Log Modules below****************************/
    pub async fn create_log(&self, log: Log) -> Result<InsertOneResult, Error> {
        let result = self
            .log
            .insert_one(log)
            .await
            .ok()
            .expect("Error inserting application or microserice");

        Ok(result)
    }

    /// Bulk insert using a single `insert_many` call
    pub async fn insert_logs_bulk(&self, logs: Vec<Log>) -> Result<(), Error> {
        if logs.is_empty() {
            return Ok(()); // nothing to insert
        }

        // Ordered=false means MongoDB will continue inserting even if one fails (e.g., duplicate key)
        let options = mongodb::options::InsertManyOptions::builder()
            .ordered(true)
            .build();

        self.log
            .insert_many(logs)
            .await
            .ok()
            .expect("error performing bulk log ");

        Ok(())
    }

    pub async fn get_logs_by_service(&self, service_id: &String) -> Result<Vec<Log>, Error> {
        let serviceid = ObjectId::from_str(service_id).expect("Failed to parse service_id");

        let filter = doc! { "my_service_id": serviceid };
        let cursor: Cursor<Log> = self
            .log
            .find(filter)
            .await
            .ok()
            .expect("error creating cusor to view documents");

        let items: Vec<Log> = cursor
            .try_collect()
            .await
            .ok()
            .expect("error reading through te logs");

        Ok(items)
    }

    pub async fn get_logs_service_by_date_range(
        &self,
        service_id: &String,
        start: DateTime,
        end: DateTime,
    ) -> Result<Vec<MyService>, Error> {
        let filter = doc! {
            "my_service_id": service_id,
            "created_at": { "$gte": start, "$lt": end }
        };

        let mut cursor: Cursor<MyService> = self
            .myservice
            .find(filter)
            .await
            .ok()
            .expect("error getting logs by id and date range");

        let services: Vec<MyService> = cursor
            .try_collect()
            .await
            .ok()
            .expect("error reading through the service list");

        Ok(services)
    }

    pub async fn delete_logs_by_date_range(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<u64, Error> {
        let filter = doc! {
            "created_at": { "$gte": start, "$lt": end }
        };

        let result = self
            .log
            .delete_many(filter)
            .await
            .ok()
            .expect("error deleting logs by date");

        Ok(result.deleted_count)
    }

    pub async fn count_by_date_range(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<u64, mongodb::error::Error> {
        let filter = doc! { "created_at": { "$gte": start, "$lt": end } };
        self.log.count_documents(filter).await
    }
}
