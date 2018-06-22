#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket::request::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

fn render(filepath: PathBuf) -> Option<Template> {
    let file = NamedFile::open(filepath);

    file.map(|mut f| {
        let mut context = HashMap::new();
        let mut content = String::new();

        f.read_to_string(&mut content).ok();
        context.insert("content", content);

        Template::render("page", context)
    }).ok()
}

#[get("/<path..>")]
fn page(path: PathBuf) -> Option<Template> {
    render(Path::new("test/pages/").join(path).join("index.txt"))
}

#[get("/")]
fn index() -> Option<Template> {
    render(Path::new("test/pages/").join("index.txt"))
}

#[error(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![page, index])
        .catch(errors![not_found])
        .attach(Template::fairing())
        .launch();
}
