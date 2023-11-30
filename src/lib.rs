use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health-check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
