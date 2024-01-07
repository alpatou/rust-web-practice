use actix_web::{App, HttpRequest, HttpServer, Responder};
mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;
use std::env;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "0");
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
