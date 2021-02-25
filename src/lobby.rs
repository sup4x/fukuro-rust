//mod ws;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap};
use crate::message_types::{ClientEvent, UserDto, Sprite, UserJoinEvent, SpriteChangeEvent, UserLeftEvent, UpdateUserPosition, ChatEvent, NodeUsersEvent, currentTime};
use uuid::Uuid;
use std::time::{Duration, Instant};

const MINUTES_TICK: Duration = Duration::from_secs(5);

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    users: HashMap<Uuid, UserDto>,
    time: HashMap<Uuid, currentTime>
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            users: HashMap::new(),
            time: HashMap::new()
        }
    }
}

impl Lobby {
    //fn defaultTime(&mut self) {
    //    //println!("{:?}",self.time);
    //    let time = "10";
    //
    //}

    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient
                .do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    #[allow(dead_code)]
    fn notify_all(&self, message: &str) {
        self.sessions.iter().for_each(
            |client| self.send_message(message, client.0)
        )
    }


    fn notify_node(&self, node: &str, message: &str ) {
        let node_users: Vec<String> = self.users.iter()
            .filter(|u| u.1.node.eq(node) ).map(|u| u.1.id.to_string()).collect();
        self.sessions.iter()
            .filter(
                |client| node_users.contains(&client.0.to_string())
            )
            .for_each(
                |client| self.send_message(message, client.0)
            )
    }

    fn init_user(&mut self, client_event: ClientEvent, uuid: Uuid) {
        let mut user_ref = self.users.get_mut(&uuid);
        let user = user_ref.as_deref_mut().unwrap();

        if client_event.name.is_some() && client_event.color.is_some() && client_event.node.is_some() {
            let name = client_event.name.unwrap();
            let color = client_event.color.unwrap();
            let node = client_event.node.unwrap();
            //let time = client_event.serverTime.unwrap();
            user.name = String::from(name.as_str());
            user.color = String::from(color.as_str());
            user.node = String::from(node.as_str());
            user.state = "active".to_string();
            user.time.dayTime = "".to_string();
            user.time.seconds = 0;
            user.time.minutes = 0;
            user.time.hours = 0;
            user.time.days = 0;
            user.time.shift = 0;
        }

    }

    fn update_user_sprite(&mut self, client_event: ClientEvent, uuid: Uuid) {
        let mut user_ref = self.users.get_mut(&uuid);
        let user = user_ref.as_deref_mut().unwrap();
        if client_event.sprite.is_some() {
            let sprite = client_event.sprite.unwrap();
            user.sprite.name = sprite.name.to_string();
            user.sprite.body = sprite.body.to_string();
            user.sprite.clothes = sprite.clothes.to_string();
            user.sprite.emotion = sprite.emotion.to_string();
            user.sprite.offset = sprite.offset.to_string();
        }
    }

    fn get_user(&mut self, uuid: Uuid) -> Option<UserDto> {
        let user = self.users.get(&uuid).as_deref().cloned();
        user
    }

    fn update_position(&mut self, uuid: Uuid, position: String) {
        let mut user_ref = self.users.get_mut(&uuid);
        let user = user_ref.as_deref_mut();
        match user {
            Some(user) => {
                user.position = position.to_owned().to_string();
            }
            _ => {}
        }
    }

    fn change_node(&mut self, uuid: Uuid, node: String) {
        let mut user_ref = self.users.get_mut(&uuid);
        let user = user_ref.as_deref_mut();
        match user {
            Some(user) => {
                user.node = node.to_owned().to_string();
            }
            _ => {}
        }
    }

    fn get_node_users(&mut self, node: &str) -> Vec<UserDto> {
        let node_users = self.users.iter()
            .filter(|u| u.1.node.eq(node))
            .map(|u| u.1)
            .cloned().collect();
        return node_users;
    }

    // fn campTime(&mut self, uuid: Uuid, ctx: &mut ws::WebsocketContext<Self>) {
    //     let mut time_ref = self.time.get_mut(&uuid);
    //     let time = time_ref.as_deref_mut().unwrap();
    //     ctx.run_interval(MINUTES_TICK, |act, ctx| {
    //         time.minutes += 1;
    //         if time.minutes == 60 {
    //             time.hours += 1;
    //             time.minutes = 0;
    //         };
    //         println!("Current time is {:?}:{:?}", time.hours, time.minutes);
    //     });
    // }
}


impl Actor for Lobby {
    type Context = Context<Self>;
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let user = self.get_user(msg.id);
        self.users.remove(&msg.id);
        let initiator = &msg.id.to_string();
        let user_left_event = UserLeftEvent {
            reason: "userLeft".to_string(),
            initiator: initiator.to_string()
        };

        match user {
            Some(user) => {
                let user_left_json = serde_json::to_string(&user_left_event).unwrap();
                self.notify_node(&user.node, &user_left_json);
            }
            _ => {}
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
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
                },
                node: "".to_string(),
                time: currentTime {
                    dayTime: "".to_string(),
                    seconds: 0,
                    minutes: 0,
                    hours: 0,
                    days: 0,
                    shift: 0
                }
            }
        );
        self.send_message(&msg.self_id.to_string(), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        let uid = msg.id;

        let client_event_json = serde_json::from_str(&msg.msg);
        if client_event_json.is_ok() {
            let client_event : ClientEvent = client_event_json.unwrap();

            let reason : &str = client_event.reason.as_str();
            match reason {
                "userInit" =>  {
                    self.init_user(client_event.to_owned(), uid);
                    self.update_user_sprite(client_event.to_owned(), uid);
                    let user = self.get_user(uid);

                    match user {
                        Some(user) => {
                            let user_join_event = UserJoinEvent {
                                reason: "userJoin".to_string(),
                                user: user.to_owned()
                            };
                            let user_join_event_json = serde_json::to_string(&user_join_event).unwrap();
                            self.notify_node(&user.node, &user_join_event_json);

                            let users = self.get_node_users(&user.node);

                            let node_users_event = NodeUsersEvent {
                                reason: "nodeUsers".to_string(),
                                users
                            };

                            let json = serde_json::to_string(&node_users_event).unwrap();
                            self.send_message(&json, &uid);
                        }
                        _ => {}
                    }

                }
                "chatMessage" => {
                    let message = client_event.message.as_deref().unwrap();
                    let sender = uid.to_string();
                    let user = self.get_user(uid);

                    match user {
                        Some(user) => {
                            let chat_event = ChatEvent {
                                reason: "chat".to_string(),
                                message: message.to_string(),
                                sender
                            };
                            let json = serde_json::to_string(&chat_event).unwrap();
                            self.notify_node(&user.node, &json);
                        }
                        _ => {}
                    }
                },
                "userMove" => {
                    let user = self.get_user(uid);

                    match user {
                        Some(user) => {
                            let position = client_event.position.as_deref().unwrap();
                            self.update_position(uid, position.to_string());
                            let update_user_position_event = UpdateUserPosition {
                                reason: "userMove".to_string(),
                                position: position.to_string(),
                                sender: user.id.to_string()
                            };
                            let json = serde_json::to_string(&update_user_position_event).unwrap();
                            self.notify_node(&user.node, &json);
                        }
                        _ => {}
                    }
                }
                "spriteChange" => {
                    self.update_user_sprite(client_event, uid);
                    let user = self.get_user(uid);

                    match user {
                        Some(user) => {
                            let sprite_change_event = SpriteChangeEvent {
                                reason: "spriteChange".to_string(),
                                user: user.to_owned()
                            };
                            let sprite_change_event_json = serde_json::to_string(&sprite_change_event)
                                .unwrap();
                            self.notify_node(&user.node, &sprite_change_event_json);
                        }
                        _ => {}
                    }
                }
                "roomChange" => {
                    if client_event.node.is_some() {
                        let node = client_event.node.as_deref().unwrap();
                        let user = self.get_user(uid.to_owned());

                        match user {
                            Some(user) => {
                                let old_node = user.node.clone();

                                self.change_node(uid, node.to_string());

                                let user_left_event = UserLeftEvent {
                                    reason: "userLeft".to_string(),
                                    initiator: user.id.to_string()
                                };

                                let fresh_user = self.get_user(uid.to_owned());
                                let json = serde_json::to_string(&user_left_event).unwrap();
                                self.notify_node(&old_node, &json);

                                match fresh_user {
                                    Some(user) => {
                                        let user_join_event = UserJoinEvent {
                                            reason: "userJoin".to_string(),
                                            user: user.to_owned()
                                        };

                                        // ниндзя код
                                        let json2 = serde_json::to_string(&user_join_event).unwrap();
                                        self.notify_node(node, &json2);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }

                        let users = self.get_node_users(node);
                        let node_users_event = NodeUsersEvent {
                            reason: "nodeUsers".to_string(),
                            users
                        };
                        let json3 = serde_json::to_string(&node_users_event).unwrap();
                        self.send_message(&json3, &uid);
                    }
                }
                _ => {
                    println!("No found message");
                }
            }
        }
    }
}
