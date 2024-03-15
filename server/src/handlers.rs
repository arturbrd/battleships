use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn test() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("")
}