#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;
use rocket::request::Request;
use std::path::{Path, PathBuf};

#[get("/<path..>")]
fn page(path: PathBuf) -> Option<NamedFile> {
    let filepath = Path::new("test/pages/").join(path).join("index.txt");
    NamedFile::open(filepath).ok()
}

#[get("/")]
fn index() -> Option<NamedFile> {
    let filepath = Path::new("test/pages/").join("index.txt");
    NamedFile::open(filepath).ok()
}

#[error(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![page, index])
        .catch(errors![not_found])
        .launch();
}
