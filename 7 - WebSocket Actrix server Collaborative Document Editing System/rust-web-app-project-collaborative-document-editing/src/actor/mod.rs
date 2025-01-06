use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;
use crate::ws_sess_manager::WsSessionManager;


// -------------------------------------------------
// Message Structs for WebSocket Events
// -------------------------------------------------

/// This message is used to notify the WsSessionManager when a new client connects
/// addr - address of the WebSocket actor representing the connection
#[derive(Message)]
#[rtype(result="()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>,
}

#[derive(Message)]
#[rtype(result="()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>,
}

// The message trait being derived will enable Connect and Disconnect to be valid messages that can be passed between the actors

//the rtype is about the actrix return type of the messages, which is an empty tuple / unit type

#[derive(Message, Clone)]
#[rtype(result="()")]
pub struct BroadcastMessage {
    pub(crate) addr: Addr<WebSocket>,
}
