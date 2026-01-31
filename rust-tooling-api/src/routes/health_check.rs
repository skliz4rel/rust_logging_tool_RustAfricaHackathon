use actix_web::{HttpResponse, Responder, get};
use dal_layer::models::response_model::GenericResponse;

#[utoipa::path(
	get,
	path = "/api/healthchecker",
	tag = "Health Checker Endpoint",
	responses(
		(status=200, description = "The health check endpoint let us konw the microservice is up and running", body = GenericResponse<String>),
	)
)]
#[get("/api/healthchecker")]
pub async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Server is running fine";

    HttpResponse::Ok().json(GenericResponse {
        code: "SERVER_IS_RUNNING".to_string(),
        data: MESSAGE.to_string(),
    })
}
