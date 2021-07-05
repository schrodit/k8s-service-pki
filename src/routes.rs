use actix_web::{get, web, Responder, HttpResponse};
use actix_web::http::{StatusCode};


pub async fn version() -> impl Responder {
    return "v0.0.1";
}