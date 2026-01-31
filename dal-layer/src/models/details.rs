use utoipa::ToSchema;

#[derive(Default, Clone, ToSchema)]
pub struct Details {
    pub name: String,
    pub age: i32,
}
