#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<file..>")]
async fn hello(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// Database connection and get, post routes for gist creation and reading
#[derive(Database)]
#[database("rustgists")]
struct Gists(sqlx::MySqlPool);

#[get("/<id>")]
async fn read(mut db: Connection<Gists>, id: i64) -> Option<String> {
   sqlx::query("SELECT content FROM gists WHERE id = ?").bind(id)
       .fetch_one(&mut *db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Gists::init())
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .mount("/gist", routes![read])
}
