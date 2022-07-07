use actix_web::HttpResponse;

pub async fn health_check_route() -> HttpResponse {
    HttpResponse::Ok().finish()
}
