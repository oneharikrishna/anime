use actix_files::NamedFile;
use actix_web::{get, web, Error, error};
use std::env;
use crate::utils::pathgenerator::generate_path;

#[get("/aot")]
async fn get_aot_image() -> Result<NamedFile, Error> {
    let aot_path = env::var("AOT").expect("AOT path should be set in the .env file");
    let result = generate_path(&aot_path);
    match result {
        Some(path) => Ok(NamedFile::open(path)?),
        None => Err(error::ErrorNotFound("No photos found"))
    }
}

pub fn aot_images(cfg: &mut web::ServiceConfig) {
    cfg.service(get_aot_image);
} 