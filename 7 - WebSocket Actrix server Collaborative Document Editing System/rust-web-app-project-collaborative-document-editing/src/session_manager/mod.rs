use crate::ws_actor::{BroadcastMessage, Connect, Disconnect, WebSocket};
use actix::{Actor, Addr, Context, Handler};
use std::{collections::HashSet, hash::Hash};

pub struct WsSessionManager {
    /// A set to track the addresses of connected WebSocket actors
    sessions: HashSet<Addr<WebSocket>>,

    //this gives it public visibility within the crate but not outside of it
    /// Stores the last text messgage broadcast to clients
    pub(crate) last_text: String,
}

impl WsSessionManager {
    pub(crate) fn new() -> Self {
        let default_text = "{\"ops\":[{\"insert\":\"Write \"},{\"attributes\":{\"underline\":true}, \"insert\":\"here\"},{\"insert\":\" some \"},{\"attributes\":{\"bold\":true},\"insert\":\"text\"},{\"insert\":\"!\"}]}";

        

        Self {
            sessions: HashSet::new(),
            last_text: default_text.to_string(),
        }
    }
}


impl Actor for WsSessionManager {
    type Context = Context<Self>;   //standard context type for Actrix actors
}

// handles the connect message that signals a new WS client has connected
impl Handler<Connect> for WsSessionManager {
    type Result = (); // no specific result is returned after handling the message

    fn handle(&mut self, msg: Connext, _: &mut Context<Self>){

        println!("New Client Connected");

        let addr = msg.addr;

        //insert the address of this new client into the sessions hash set so the Ws Session Manager can track them all
        self.sessions.insert(addr.clone());
    }
}