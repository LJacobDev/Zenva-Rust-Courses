/*

    NOTE:

        this project was made following a video course that is from 2020, so it was using an older version of actix that is so far behind the new current version that the syntax won't compile in the new version, and the old version's dependencies are no longer available (ring 0.12.0), so for now it only seems possible to code along with this and have it as a way to learn examples of how web servers run, even though actix itself seems to have changed from what this is shown to be like

        it is not expected that this server can compile or run or anything, but the code blocks can still be read and understood for general knowledge about web applications or at least how they were in 2020

*/

mod session_manager;
mod actor;

use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./public").index_file("index.html"));
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



