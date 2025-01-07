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

/// A message to be broadcast to all connected WebSocket clients
/// msg = the content that will be sent
/// sender = the address of the WS actor that sent the message
#[derive(Message, Clone)] // Clone is derived to allow easy duplicating of the message for broadcasting
#[rtype(result="()")]
pub struct BroadcastMessage {
    pub(crate) msg: String,
    pub(crate) sender: Addr<WebSocket>,
}

impl Handler<BroadcastMessage> for WebSocket {
    type Result = ();

    // the instructor says that this function sends the broadcast text to the intended client
    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(mes.text);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DefaultMessage {
    pub text: String,
}

impl Handler<DefaultMessage> for WebSocket {
    type Result = ();

    // the instructor says that this function sends the default text to the intended client
    fn handle(&mut self, msg: DefaultMessage, ctx: &mut Self::Context) {
        ctx.text(mes.text);
    }
}



pub struct WebSocket {

    // Address of the websocket manager, which manages all of the websocket connections
    // it needs to implement the Actor trait so that it can be used as an Actix actor in the whole application
    pub(crate) manager: Adds<WsSessionManager>,

}


impl Actor for WebSocket {

    //defines the type of context used by the websocket actor
    type Context = ws::WebSocketContext<Self>;


    // handle the events of when the websocket connections are started / registered and stopped / unregistering the connection from the server
    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.do_send(Connect {
            addr: ctx.address(), //send the address of this WS actor to the session manager
        })
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.manager.do_send(Disconnect {
            addr: ctx.address(),
        });
        Running::Stop   // indicates to the actor that the runtime should stop
    }

}