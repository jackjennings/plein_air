#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate autolink;
extern crate rocket_contrib;

use autolink::auto_link;
use rocket::fairing::AdHoc;
use rocket::request::Request;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

struct Configuration {
    content_directory: String
}

fn render(filepath: PathBuf) -> Option<Template> {
    let file = NamedFile::open(filepath);

    file.map(|f| Template::render("page", context_for(f))).ok()
}

fn context_for(mut file: NamedFile) -> HashMap<String, String> {
    let mut content = String::new();
    file.read_to_string(&mut content).ok();

    let title = content.lines().next().unwrap_or("").to_string();

    let mut context = HashMap::new();
    context.insert(String::from("title"), title);
    context.insert(String::from("content"), auto_link(&content, &[]));
    context
}

#[get("/<path..>")]
fn page(path: PathBuf, configuration: State<Configuration>) -> Option<Template> {
    let root = &configuration.content_directory;
    render(Path::new(root).join(path).join("index.txt"))
}

#[get("/")]
fn index(configuration: State<Configuration>) -> Option<Template> {
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
                content_directory: content_directory.to_string()
            };

            Ok(rocket.manage(configuration))
        }))
        .attach(Template::fairing())
        .launch();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_for() {
        let file = NamedFile::open("./test/pages/foo/index.txt");
        let mut expected = HashMap::new();
        expected.insert(String::from("title"), String::from("Hell world!"));
        expected.insert(String::from("content"), String::from("Hell world!\n"));

        assert_eq!(expected, context_for(file.unwrap()))
    }
}
