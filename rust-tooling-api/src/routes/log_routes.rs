use actix_web::{
    Error, HttpResponse,
    web::{Data, Json, Path},
};
use actix_web::{get, post};
use dal_layer::{
    models::{
        log_model::{Log, LogRequest},
        my_service_model::MyServiceView,
        response_model::GenericResponse,
    },
    repository::db::Database,
    utils::date_helper::Converter,
};
use mongodb::bson::DateTime;

#[utoipa::path(
	post,
	path = "/api/log",
    tag = "Register Logs from microservice log files",
	request_body(content= LogRequest, description="Store the logs from log files", example= json!({"email":"skliz4rel@gmail.com", "password":"password"})),
	responses(
		(status=201, description="Log was successfully sent", body=GenericResponse<LogRequest>),
		(status=500, description="Internal Server Error while trying to send the logs", body= GenericResponse<String>)
	)
	)]
#[post("/api/log")]
pub async fn create_log(db: Data<Database>, request: Json<LogRequest>) -> HttpResponse {
    match db
        .create_log(
            Log::try_from(LogRequest {
                level: request.level.clone(),
                my_service_id: request.my_service_id.clone(),
                line_content: request.line_content.clone(),
                created_at: request.created_at.clone(),
            })
            .expect("Error converting DogRequest to Dog."),
        )
        .await
    {
        Ok(logs) => HttpResponse::Ok().json(GenericResponse {
            code: String::from("200"),
            data: logs,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
	get,
	path = "/api/logs/{service_id}",
	tag = "View logs by id of the microservice",
	responses(
		(status=200, description = "This is going to be used to view the logs by the id of the microservice", body = GenericResponse<String>),
	)
	)]
#[get("/api/logs/{service_id}")]
pub async fn get_logs_byservices(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let service_id: String = path.into_inner().0;
    match db.get_logs_by_service(&service_id).await {
        Ok(services) => HttpResponse::Ok().json(GenericResponse {
            code: String::from("200"),
            data: services,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
	get,
	path = "/api/logs/bydate/{service_id}/{start_date}/{end_date}",
	tag = "View logs by date and service id",
	responses(
		(status=200, description = "This shows the microservice is up and running", body = GenericResponse<Vec<MyServiceView>>),
	)
)]
#[get("/api/logs/bydate/{service_id}/{start_date}/{end_date}")]
pub async fn get_logs_services_by_date_range(
    db: Data<Database>,
    path: Path<(String, String, String)>,
) -> HttpResponse {
    let (service_id, start_date, end_date) = &path.into_inner();

    let start_date: DateTime = Converter::convert_str_datetime(&start_date);
    let end_date: DateTime = Converter::convert_str_datetime(&end_date);

    match db
        .get_logs_service_by_date_range(service_id, start_date, end_date)
        .await
    {
        Ok(services) => HttpResponse::Ok().json(GenericResponse {
            code: String::from("200"),
            data: services,
        }),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
