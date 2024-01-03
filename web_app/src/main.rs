use actix_web::{ App, HttpServer, Responder, HttpRequest};
mod views;
mod to_do;
mod state;
mod processes;

async fn greet ( req: HttpRequest) -> impl Responder {
    let name  = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app  = App::new().configure(views::views_factory);
        return app
        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

