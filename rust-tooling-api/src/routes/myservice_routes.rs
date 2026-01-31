use actix_web::{
    Error, HttpResponse,
    error::HttpError,
    web::{Data, Json},
};
use actix_web::{get, post};

use mongodb::bson::Bson;

use dal_layer::{
    models::{
        my_service_model::{MyService, MyServiceView},
        response_model::GenericResponse,
    },
    repository::db::Database,
};

#[utoipa::path(
	post,
    tag = "Register Service",
	path = "/api/service",
	request_body(content= MyServiceView, description="End point for saving the microservice which needs log tracking", example= json!({
    "name":"payment",
     "description":"This is the payment service",
    "onboarded_datetime":"2024-05-30T10:00:00.000Z"
})),
	responses(
		(status=200, description="Service successfully saved", body=GenericResponse<String>),
		(status=400, description="BadRequest when saving the service", body=GenericResponse<String>),
		(status=500, description="Internal Server Error", body= GenericResponse<String>)
	)
	)]
#[post("/api/service")]
pub async fn create_service(db: Data<Database>, request: Json<MyServiceView>) -> HttpResponse {
    match db
        .create_service(
            MyService::try_from(MyServiceView {
                service_id: None,
                name: request.name.clone(),
                description: request.description.clone(),
                onboarded_datetime: request.onboarded_datetime.clone(),
            })
            .expect("Error converting Service request to Serviice entity."),
        )
        .await
    {
        Ok(objectid) => {
            let id_str = match objectid.inserted_id {
                Bson::ObjectId(oid) => oid.to_hex(),
                _ => String::new(), // fallback if something weird happens
            };

            HttpResponse::Ok().json(GenericResponse {
                code: String::from("200"),
                data: id_str,
            })
        }
        Err(err) => HttpResponse::InternalServerError().json(GenericResponse {
            code: String::from("500"),
            data: err.to_string(),
        }),
    }
}

#[utoipa::path(
	get,
	path = "/api/services",
	tag = "Get Service",//"Display registered Microservices",
	responses(
		(status=200, description = "This is going to display the list of registered microservices running", body = GenericResponse<Vec<MyServiceView>>),
	)
)]
#[get("/api/services")]
pub async fn get_services(db: Data<Database>) -> HttpResponse {
    match db.get_services().await {
        Ok(services) => HttpResponse::Ok().json(GenericResponse {
            code: String::from("200"),
            data: services,
        }),
        Err(err) => HttpResponse::InternalServerError().json(GenericResponse {
            code: String::from("500"),
            data: err.to_string(),
        }),
    }
}
