use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Sprite {
    // body: i8,
    // emotion: i8,
    // cloth: i8,
    // accessory: i8,
    pub body : Option<String>,
    pub emotion : Option<String>,
    pub cloth : Option<String>,
    pub accessory : Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    // pub id : Option<String>,
    // pub state : Option<String>,
    // pub name : Option<String>,
    // pub color : Option<String>,
    // pub sprite : Option<Sprite>,
    // pub position: Option<String>
    pub(crate) id : String,
    pub(crate) state : String,
    pub(crate) name : String,
    pub(crate) color : String,
    // pub sprite : Option<Sprite>,
    pub(crate) position: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerEvent  {
    pub reason: String,
    pub user: Option<UserDto>,
    // id: i32,
    pub sender: Option<String>,
    pub message: Option<String>,
    pub code: Option<String>,
    pub actionCode: Option<String>,
    pub users: Option<Vec<UserDto>>,
}

// impl ServerEvent {
//     pub fn new(reason: &str) -> ServerEvent {
//         return ServerEvent {
//             reason: String::from_string(&reason),
//         }
//     }
// }
