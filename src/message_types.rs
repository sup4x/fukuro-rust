use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Sprite {
    // body: i8,
    // emotion: i8,
    // cloth: i8,
    // accessory: i8,
    pub name: String,
    pub body : String,
    pub clothes : String,
    pub emotion : String,
    pub offset : String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct ClientEvent {
    pub reason : String,
    pub id: Option<i64>,
    pub name : Option<String>,
    pub color : Option<String>,
    pub character: Option<String>,
    pub node: Option<String>,
    pub sprite : Option<Sprite>,
    pub target : Option<String>,
    pub position : Option<String>,
    pub message: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct UserDto {
    pub id : String,
    pub state : String,
    pub name : String,
    pub color : String,
    pub sprite : Sprite,
    pub position: String,
    pub node: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJoinEvent {
    pub reason: String,
    pub user: UserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpriteChangeEvent {
    pub reason: String,
    pub user: UserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLeftEvent {
    pub reason: String,
    pub initiator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPosition {
    pub reason: String,
    pub position: String,
    pub sender: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUserNode {
    pub reason: String,
    pub node: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatEvent {
    pub reason: String,
    pub message: String,
    pub sender: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeUsersEvent {
    pub reason: String,
    pub users: Vec<UserDto>,
}
