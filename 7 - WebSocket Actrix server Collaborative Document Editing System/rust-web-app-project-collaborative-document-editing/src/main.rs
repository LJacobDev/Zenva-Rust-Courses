/*

    NOTE:

        this project was made following a video course that is from 2020, so it was using an older version of actix that is so far behind the new current version that the syntax won't compile in the new version, and the old version's dependencies are no longer available (ring 0.12.0), so for now it only seems possible to code along with this and have it as a way to learn examples of how web servers run, even though actix itself seems to have changed from what this is shown to be like

        it is not expected that this server can compile or run or anything, but the code blocks can still be read and understood for general knowledge about web applications or at least how they were in 2020

*/

/*
    Note 2:

        the video course about websockets used the following code to demonstrate that an actix server could be started up, but it ended up removing all of this main function content and it replaced it with some other kind of thing that seemed to be needed to make the websockets work
*/

/*
mod session_manager;
mod actor;
mod errors;

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
 */

/*

// // this is supposed to be how to start one of these in actix 4.9.0
   // but the other module files didn't work so I'll do the old way all through this project
   // and then I can try remaking it later in actix 4.9.0 ways of doing it

// use actix_files::Files;
// use actix_web::{web, App, HttpServer, HttpResponse};

// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello Wold!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new( || {
//         App::new()
//             .service(Files::new("/static", "./public"))
//             .route("/", web::get().to(index))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
 */

/*
    Note 3:

    The following is what was written out for the websocket main.rs file by lesson 12


    It has an asynchronous function to handle the incoming WebSocket connection requests

    it creates a websocket actor instance for every connection that is available,

    and starts WebSocket session for each of them

    and connects to the main server


    It will be an 'Actix' web application,

        it starts with an http request from a client and then elevates the connected user to WebSocket type conneciton


*/

mod actor;
mod errors;
mod session_manager;

use crate::actor::WebSocket;
use crate::errors::ConnectionError;
use crate::session_manager::WsSessionManager;
use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

/// an async function that handles incoming websocket requests
/// it will have requests, stream, and server instance parameters to handle
/// and it creates a WebSocket actor instance and starts the WebSocket connection
async fn ws_index(
    req: HttpRequest,
    stream: web::PayLoad,
    server_instance: web::Data<Addr<WsSessionManager>>,
) -> Result<HttpResponse, Error> {
    // create WebSocket Actor instance
    let ws_actor = WebSocket {
        manager: server_instance.get_ref().clone(),
    };

    // start the websocket connection
    // upgrade the HTTP connection to a WebSocket connection
    // initialize the WebSocket Actor to handle messages on the new connection

    ws::start(ws_actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // create the manager
    // provide the main address for the web application
    // and construct the Actix web application
    // also use the errors module to handle server creation errors

    // create and start the WsSessionManager Actor
    let manager = WsSessionManager::new().start();

    // provide the main address for the web application
    // 0.0.0.0 listens not only to localhost, but also to public IP address incoming traffic, and virtual private network traffic, and Ethernet / Wifi connected device requests too
    let address = "0.0.0.0:8080";

    // create the Actix application that uses an HttpServer
    let server = HttpServer::new(move || {
        App::new()
            // a GET request going to /ws/ will be routed to the ws_index function as its endpoint
            .route("/ws/", web::get().to(ws_index)) //websocket route
            // a 'request' to "/" will go to the public index.html file, but not sure why it has .service() instead of being another .route()
            // it seems to be because it is the' serving static file'
            .service(actix_files::Files::new("/", "public").index_file("index.html"))  
            // share the Session Manager across the whole application
            // I'm not sure why it's cloning this if it's putting it in the app_data area
            // I'd have thought it would be able to move it directly into that, so I'd like to know more about this
            .app_data(manager.clone())
    })
    .workers(1)
    .bind(address);

    // this server needs to be checked in a match statement for either an Ok(server) or Err()
    match server {
        Ok(srv) => {
            println!("Server is listening on: {}", address);
            srv.run().await
        },
        Err(error) => {
            panic!("{}", ConnectionError::CreateServerError(error.to_string()))
        }
    }

}
