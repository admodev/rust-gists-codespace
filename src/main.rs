#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<file..>")]
async fn hello(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
}
