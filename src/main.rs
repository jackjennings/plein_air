#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use rocket::request::Request;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

struct ContentDirectory(String);

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
fn page(path: PathBuf, content_directory: State<ContentDirectory>) -> Option<Template> {
    let root = &content_directory.0;
    render(Path::new(root).join(path).join("index.txt"))
}

#[get("/")]
fn index(content_directory: State<ContentDirectory>) -> Option<Template> {
    let root = &content_directory.0;
    render(Path::new(root).join("index.txt"))
}

#[error(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![page, index])
        .catch(errors![not_found])
        .attach(AdHoc::on_attach(|rocket| {
            let content_directory = match rocket.config().get_str("content_directory") {
                Ok(dir) => dir.to_string(),
                Err(_e) => panic!("must set content directory"),
            };

            Ok(rocket.manage(ContentDirectory(content_directory.to_string())))
        }))
        .attach(Template::fairing())
        .launch();
}
