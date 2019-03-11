use askama::Template;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::content::Content;
use rocket::response::{self, Responder};

#[derive(Template)]
#[template(path = "page.html")]
pub struct Page {
    pub content: String,
    pub title: String,
}

impl<'r> Responder<'r> for Page {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Content(ContentType::HTML, self.render()).respond_to(req)
    }
}
