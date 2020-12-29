use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::message_types::{ServerEvent, ClientEvent, UserDto};
use actix_web::web::Json;


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

        let disconnectStr = " sebal";
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
                position: "".to_string()
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
        // let eventMsg: ServerEvent = serde_json::from_str(&text).unwrap();
        let clientEvent: ClientEvent = serde_json::from_str(&msg.msg).unwrap();
        // println!("client struct {:?}", clientEvent);
        // println!("client msg {}", msg.msg);
        let reason : &str = clientEvent.reason.as_str();
        match reason {
            "userInit" =>  {
                // self.users.get()
                let nameVa  = clientEvent.name.unwrap();
                let nameStr = nameVa.as_str();
                let colorVa  = clientEvent.color.unwrap();
                let colorStr = colorVa.as_str();
                let mut userRef = self.users.get_mut(&uid);
                let user = userRef.as_deref_mut().unwrap();
                // user.name = Option::from(String::from(nameStr));
                // user.color = Option::from(String::from(colorStr));
                user.name = String::from(nameStr);
                user.color = String::from(colorStr);

                let userJson = serde_json::to_string(user).unwrap();
                let jsonStr = userJson.as_str();

                // println!("User Data");
                // println!("User Data {:?}", jsonStr);

                let chatEventStr = "{ \"reason\": \"userJoin\", \"user\": ".to_string() + jsonStr + " }";

                self.sessions
                    // .unwrap()
                    .iter().
                    // for_each(|client| self.send_message("{\"reason\": \"userJoin\"}",  client.0))
                    for_each(|client| self.send_message(chatEventStr.as_str(),  client.0));
                // self.send_message()

                let users = self.users.values().cloned().collect::<Vec<UserDto>>();
                // let userJson: Vec<String> = users.iter().map(|u|  serde_json::to_string(&u).unwrap().).collect();
                // let users : Vec<UserDto> = self.users.into_iter().map(|u| u.1).collect();
                // println!("users {:?}", users);
                // println!("userJson {:?}", userJson);

                let usersData = serde_json::to_string(&users).unwrap();
                let userDataStr = usersData.as_str();
                let nodeDataStr = "{ \"reason\": \"usersData\", \"users\": ".to_string() + userDataStr + " }";

                self.send_message(nodeDataStr.as_str(), &uid)

            }
            "chatMessage" => {
                // let chatEvent = ServerEvent {
                //     reason: String::from("chat"),
                //     user: None,
                //     sender: None,
                //     message: None,
                //     code: None,
                //     actionCode: None,
                //     users: None
                // };

                // msg.msg.message
                // println!("{:?}", uid);
                let message = clientEvent.message.as_deref().unwrap();
                let sender = uid.to_string();
                let senderStr = sender.as_str();
                let chatEventMsg  = "{\"reason\": \"chat\", \"message\": \"".to_string() + message + "\", \"sender\": \"" + senderStr  + "\"}";

                self.sessions.iter().for_each(
                    |client| self.send_message(chatEventMsg.as_str(), client.0)
                )
            }
            _ => {
                println!("No found message");
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
