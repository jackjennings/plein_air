#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate askama;

mod format;
mod page;

use page::Page;
use rocket::fairing::AdHoc;
use rocket::request::Request;
use rocket::response::NamedFile;
use rocket::State;
use std::io::Read;
use std::path::{Path, PathBuf};

struct Configuration {
    content_directory: String,
}

fn render(filepath: PathBuf) -> Option<Page> {
    let mut file = NamedFile::open(filepath).ok()?;
    let mut content = String::new();
    file.read_to_string(&mut content).ok();
    let title = content.lines().next().unwrap_or("").to_string();

    Some(Page {
        content: format::autolink(&content),
        title: title,
    })
}

#[get("/<path..>")]
fn page(path: PathBuf, configuration: State<Configuration>) -> Option<Page> {
    let root = &configuration.content_directory;
    render(Path::new(root).join(path).join("index.txt"))
}

#[get("/")]
fn index(configuration: State<Configuration>) -> Option<Page> {
    let root = &configuration.content_directory;
    render(Path::new(root).join("index.txt"))
}

#[catch(404)]
fn not_found(_req: &Request) -> &'static str {
    "Not Found"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![page, index])
        .register(catchers![not_found])
        .attach(AdHoc::on_attach("Content Config", |rocket| {
            let content_directory = match rocket.config().get_str("content_directory") {
                Ok(dir) => dir.to_string(),
                Err(_e) => panic!("must set content directory"),
            };
            let configuration = Configuration {
                content_directory: content_directory.to_string(),
            };

            Ok(rocket.manage(configuration))
        }))
        .launch();
}
