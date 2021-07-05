use actix_web::{get, web, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    format!("Hello World")
}
