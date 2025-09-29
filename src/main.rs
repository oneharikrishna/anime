use actix_web::{HttpServer, App};
use dotenvy::dotenv;
mod handlers;
mod utils;
use handlers::{demonslayer,onepiece,aot};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(||{
        App::new()
            .configure(demonslayer::demonslayer_images)
            .configure(onepiece::onepiece_images)
            .configure(aot::aot_images)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await;
    Ok(())
}