// use actix::{Actor, StreamHandler, Addr, Handler};
// use actix::prelude::*;
// use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, get};
// use actix_web_actors::ws;
// use serde::{Deserialize, Serialize, Serializer};
// use json::JsonValue;

mod ws;
mod lobby;
use lobby::Lobby;
mod messages;
mod start_connection;
mod message_types;
// mod start_connection;

use start_connection::start_connection as start_connection_route;
use actix::Actor;
use actix_web::{App, HttpServer, web, Responder, HttpResponse, get};
use std::env;



#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO вернуть перед продом
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let chat_server = Lobby::default().start(); //create and spin up a lobby

    // HttpServer::new(move || {
    //     App::new()
    //         // .service(start_connection_route) //. rename with "as" import or naming conflict
    //         .route("/", web::get().to(start_connection))
    //         .data(chat_server.clone()) //register the lobby
    // })
    //     .bind("127.0.0.1:8081")?
    //     .run()
    //     .await

    HttpServer::new(
        move || App::new()
            .service(hello)
            .service(start_connection_route)
            .data(chat_server.clone())
    )
        // .bind("127.0.0.1:8081")?
        // TODO вернуть перед продом
        .bind(("0.0.0.0", port))
        .expect("Can not bind to port 8000")
        .run()
        .await
}




// struct User {}
//
// struct Sprite {
//     body: i8,
//     emotion: i8,
//     cloth: i8,
//     accessory: i8,
// }
//
// struct ClientEvent {
//     reason: str,
//     id: str,
//     name: str,
//     color: str,
//     character: str,
//     node: str,
//     sprite: Sprite,
//     target: str,
//     position: i64,
//     message: str
// }
//
// struct UserDto {
//     id: i8,
//     state: str,
//     name: str,
//     color: str,
//     sprite: Sprite,
//     position: str
// }
//
// struct ServerEvent  {
//     reason: str,
//     user: UserDto,
//     id: str,
//     sender: i8,
//     message: str,
//     code: str,
//     actionCode: str,
//     users: Vec<UserDto>,
// }

// Define HTTP actor
// struct ServerMonitor {
//     listeners: Vec<Addr<MyWebSocket>>;
// }
//
// impl Actor for ServerMonitor {
//     type Context = ws::WebsocketContext<Self>;
//
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self::users = Vec::new();
//     }
// }
//
// impl Handler<RegisterWSClient> for ServerMonitor {
//     type Result = ();
//
//     fn handle(&mut self, msg: RegisterWSClient, _: &mut Context<Self>) {
//         self.listeners.push(msg.addr);
//     }
// }
//
// /// Handler for ws::Message message
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ServerMonitor {
//     fn handle(
//         &mut self,
//         msg: Result<ws::Message, ws::ProtocolError>,
//         ctx: &mut Self::Context,
//     ) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => {
//                 ctx.pong(&msg);
//                 ctx.text("PONG YOPTA");
//             },
//             Ok(ws::Message::Text(text)) => {
//                 let serverEvent: ServerEvent = serde_json::from_str(&text).unwrap();
//
//                 match serverEvent.reason {
//                    "userInit" => {
//                        // ctx.
//                    }
//                 }
//                 // let event: Result<ChatEvent, E> = serde_json::from_str(&text);
//                 // println!("{:?}", event);
//
//                 // match event {
//                     // Ok(event) => {
//                     //     let messageJson = serde_json::to_string(&event);
//                     //     match messageJson {
//                     //         Ok(v) => {
//                     //             ctx.text(v);
//                     //             ctx.text(text);
//                     //         }
//                     //     }
//                     // }
//                 // }
//                 //
//                 // if messageJson.is_ok() {
//                 //     ctx.text(messageJson.context());
//                 // }
//                 // //
//                 // ctx.text("sdfsdfsdf");
//                 // println!("sdfsdfffff");
//                 // ctx.text(text)
//             },
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             _ => (),
//         }
//     }
// }
//
// async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(MyWs {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }
//
// #[get("/hello")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(hello).route("/", web::get().to(index)))
//         .bind("127.0.0.1:8081")?
//         .run()
//         .await
// }
