use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler};

pub struct WsSessionManager{
    /// A set to track the addresses of connected WebSocket actors
    sessions: HashSet<Addr<WebSocket>>,

    //this gives it public visibility within the crate but not outside of it
    /// Stores the last text messgage broadcast to clients
    pub(crate) last_text: String,
}

impl WsSessionManager {

}