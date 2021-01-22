mod ws;
mod lobby;
use lobby::Lobby;
mod messages;
mod start_connection;
mod message_types;

use start_connection::start_connection as start_connection_route;
use actix::Actor;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("PORT must be a number");
    println!("Server is working on {:?}", port);
    let chat_server = Lobby::default().start(); //create and spin up a lobby
    HttpServer::new(
        move || App::new()
            .service(start_connection_route)
            .data(chat_server.clone())
    )
        .bind(("0.0.0.0", port))
        .expect("Can not bind to port")
        .run()
        .await
}
