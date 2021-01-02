use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::message_types::{ServerEvent, ClientEvent, UserDto, Sprite};
use actix_web::web::Json;
use json::JsonError;


type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    users: HashMap<Uuid, UserDto>
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            // rooms: HashMap::new(),
            users: HashMap::new()
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    // Добавить отправку событий в комнату
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.users.remove(&msg.id);
        // self.sessions.remove(&msg.id);


        let initiator = &msg.id.to_string();
        let initiatorStr = initiator.as_str();
        let userLeaveEvent = "{ \"reason\": \"userLeft\", \"initiator\": \"".to_string() + initiatorStr + "\"}";

        self.sessions
            .iter().
            for_each(|client| self.send_message(userLeaveEvent.as_str(),  client.0));
        // let disconnectStr = " sebal";
        // println!("sessions {:?}" self.rooms);
        // println!("sessions {:?}" self.sessions);
        // self.sessions
            // .unwrap()
            // .iter().
            // for_each(|client| self.send_message("{\"reason\": \"userJoin\"}",  client.0))
            // for_each(|client| self.send_message(disconnectStr,  client.0));
        //     self.rooms
        //         .get(&msg.room_id)
        //         .unwrap()
        //         .iter()
        //         .filter(|conn_id| *conn_id.to_owned() != msg.id)
        //         .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
        //     if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
        //         if lobby.len() > 1 {
        //             lobby.remove(&msg.id);
        //         } else {
        //             //only one in the lobby, remove it entirely
        //             self.rooms.remove(&msg.room_id);
        //         }
        //     }
        // }
    }
}


impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        // self.rooms
            // .entry(msg.lobby_id)
            // .or_insert_with(HashSet::new).insert(msg.self_id);

        // send to everyone in the room that new uuid just joined
        // self
        //     .rooms
        //     .get(&msg.lobby_id)
        //     .unwrap()
        //     .iter()
        //     .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
        //     .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

        // store the address
        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );

        let id  = msg.self_id.to_string();
        self.users.insert(
            msg.self_id,
            UserDto {
                id,
                state: "spectating".to_string(),
                name: "".to_string(),
                color: "".to_string(),
                position: "0".to_string(),
                sprite: Sprite {
                    name: "".to_string(),
                    body: "".to_string(),
                    clothes: "".to_string(),
                    emotion: "".to_string(),
                    offset: "".to_string()
                }
            }
        );

        // send self your new uuid
        // self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
        self.send_message(&msg.self_id.to_string(), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        let uid = msg.id;

        // let clientEvent: ClientEvent = serde_json::from_str(&msg.msg).unwrap();
        let clientEventJson = serde_json::from_str(&msg.msg);
        if clientEventJson.is_ok() {
            let clientEvent : ClientEvent = clientEventJson.unwrap();

            let reason : &str = clientEvent.reason.as_str();
            match reason {
                "userInit" =>  {
                    let nameVa  = clientEvent.name.unwrap();
                    let nameStr = nameVa.as_str();
                    let colorVa  = clientEvent.color.unwrap();
                    let colorStr = colorVa.as_str();
                    let mut userRef = self.users.get_mut(&uid);
                    let user = userRef.as_deref_mut().unwrap();
                    user.name = String::from(nameStr);
                    user.color = String::from(colorStr);

                    let spriteOptional = clientEvent.sprite;
                    if spriteOptional.is_some() {
                        let sprite = spriteOptional.unwrap();

                        user.sprite.name = sprite.name.to_string();
                        user.sprite.body = sprite.body.to_string();
                        user.sprite.clothes = sprite.clothes.to_string();
                        user.sprite.emotion = sprite.emotion.to_string();
                        user.sprite.offset = sprite.offset.to_string();


                        let userJson = serde_json::to_string(user).unwrap();
                        let jsonStr = userJson.as_str();


                        let chatEventStr = "{ \"reason\": \"userJoin\", \"user\": ".to_string() + jsonStr + " }";

                        self.sessions
                            .iter().
                            for_each(|client| self.send_message(chatEventStr.as_str(),  client.0));

                        let users = self.users.values().cloned().collect::<Vec<UserDto>>();

                        let usersData = serde_json::to_string(&users).unwrap();
                        let userDataStr = usersData.as_str();
                        let nodeDataStr = "{ \"reason\": \"usersData\", \"users\": ".to_string() + userDataStr + " }";

                        self.send_message(nodeDataStr.as_str(), &uid)
                    }

                }
                "chatMessage" => {
                    let message = clientEvent.message.as_deref().unwrap();
                    let sender = uid.to_string();
                    let senderStr = sender.as_str();
                    let chatEventMsg  = "{\"reason\": \"chat\", \"message\": \"".to_string() + message + "\", \"sender\": \"" + senderStr  + "\"}";

                    self.sessions.iter().for_each(
                        |client| self.send_message(chatEventMsg.as_str(), client.0)
                    )
                },
                "userMove" => {
                    let position = clientEvent.position.as_deref().unwrap();
                    let sender = uid.to_string();

                    let senderStr = sender.as_str();
                    let moveUserStr  = "{\"reason\": \"userMove\", \"position\": \"".to_string() + position + "\", \"sender\": \"" + senderStr  + "\"}";

                    self.sessions.iter().for_each(
                        |client| self.send_message(moveUserStr.as_str(), client.0)
                    )
                }
                "spriteChange" => {
                    let mut userRef = self.users.get_mut(&uid);
                    let user = userRef.as_deref_mut().unwrap();

                    // let sprite = clientEvent.sprite.unwrap();
                    let spriteOption = clientEvent.sprite;
                    if spriteOption.is_some() {
                        let sprite = spriteOption.unwrap();

                        user.sprite.name = sprite.name.to_string();
                        user.sprite.body = sprite.body.to_string();
                        user.sprite.clothes = sprite.clothes.to_string();
                        user.sprite.emotion = sprite.emotion.to_string();
                        user.sprite.offset = sprite.offset.to_string();


                        let userJson = serde_json::to_string(user).unwrap();
                        let jsonStr = userJson.as_str();


                        let chatEventStr = "{ \"reason\": \"spriteChange\", \"user\": ".to_string() + jsonStr + " }";

                        self.sessions
                            // .unwrap()
                            .iter().
                            // for_each(|client| self.send_message("{\"reason\": \"userJoin\"}",  client.0))
                            for_each(|client| self.send_message(chatEventStr.as_str(),  client.0));
                    }

                }
                _ => {
                    println!("No found message");
                }
            }
        }
        // if msg.msg.starts_with("\\w") {
        //     if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
        //         self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
        //     }
        // } else {
        //
        //     self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        // }
    }
}
