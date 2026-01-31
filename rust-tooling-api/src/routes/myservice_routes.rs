use actix_web::{
    Error, HttpResponse,
    web::{Data, Json},
};
use actix_web::{get, post};

use dal_layer::{
    models::{
        my_service_model::{MyService, MyServiceView},
        response_model::GenericResponse,
    },
    repository::db::Database,
};

#[utoipa::path(
	post,
    tag = "Register Microservices that you want to store logs for",
	path = "/api/service",
	request_body(content= MyServiceView, description="End point for saving the microservice which needs log tracking", example= json!({"email":"skliz4rel@gmail.com", "password":"password"})),
	responses(
		(status=200, description="Service successfully saved", body=GenericResponse<MyServiceView>),
		(status=400, description="BadRequest when saving the service", body=GenericResponse<MyServiceView>),
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
        Ok(myservice) => HttpResponse::Ok().json(GenericResponse {
            code: String::from("200"),
            data: myservice,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
	get,
	path = "/api/services",
	tag = "Display registered Microservices",
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
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
