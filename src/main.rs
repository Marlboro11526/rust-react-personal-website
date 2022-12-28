use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

mod utils;

#[derive(Serialize)]
struct HomePageProps {
    message: String
}

#[get("/")]
async fn show_home() -> impl Responder {
    let component = String::from("Home");
    let props = HomePageProps {
        message: String::from("Hello World!"),
    };
    HttpResponse::Ok().body(utils::renderer::render_with_props(component, props))
}
    
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(show_home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}