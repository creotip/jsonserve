#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;
mod paste_id;

use std::io;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use paste_id::PasteId;


// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");
const ID_LENGTH: usize = 5;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    let path = id.file_path();

    // Create directories if they do not exist
    if let Some(dir_path) = path.parent() {
        fs::create_dir_all(dir_path).await?;
        fs::set_permissions(dir_path, Permissions::from_mode(0o755)).await?;
    }

    match paste.open(128.kibibytes()).into_file(&path).await {
        Ok(_) => Ok(uri!(HOST, retrieve(id)).to_string()),
        Err(e) => {
            eprintln!("Failed to save file: {:?}", e); // Log the error to stderr
            Err(e)
        }
    }
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/favicon.ico")]
fn favicon() -> Option<()> {
    None
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, delete, retrieve, favicon]).configure(rocket::Config{
        address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),  // Bind to all addresses
        port: 8000,
        ..rocket::Config::default()
    })
}