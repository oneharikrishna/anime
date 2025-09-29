use actix_files::NamedFile;
use actix_web::{get, web, Error, error};
use std::env;
use crate::utils::pathgenerator::generate_path;

#[get("/onepiece")]
async fn get_onepiece_image() -> Result<NamedFile, Error> {
    let onepiece_path = env::var("ONEPIECE").expect("ONEPIECE path should be set in the .env file");
    let result = generate_path(&onepiece_path);
    match result {
        Some(path) => Ok(NamedFile::open(path)?),
        None => Err(error::ErrorNotFound("No Photos Found"))
    }
}

pub fn onepiece_images(cfg: &mut web::ServiceConfig) {
    cfg.service(get_onepiece_image);
}