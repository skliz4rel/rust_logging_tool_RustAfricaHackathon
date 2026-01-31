use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GenericResponse<T> {
    pub code: String,
    pub data: T,
}
