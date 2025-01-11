use crate::sess_manager::WsSessionManager;  //he just changed it from ws_session_manager to session_manager in lesson 12 with no explanation as to why, but I think it was just him correcting his error because session_manager::WsSessionManager makes sense where the other one did not
use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;

// -------------------------------------------------
// Message Structs for WebSocket Events
// -------------------------------------------------

/// This message is used to notify the WsSessionManager when a new client connects
/// addr - address of the WebSocket actor representing the connection
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>,
}

// The message trait being derived will enable Connect and Disconnect to be valid messages that can be passed between the actors

//the rtype is about the actrix return type of the messages, which is an empty tuple / unit type

/// A message to be broadcast to all connected WebSocket clients
/// msg = the content that will be sent
/// sender = the address of the WS actor that sent the message
#[derive(Message, Clone)] // Clone is derived to allow easy duplicating of the message for broadcasting
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub(crate) msg: String,
    pub(crate) sender: Addr<WebSocket>,
}

impl Handler<BroadcastMessage> for WebSocket {
    type Result = ();

    // the instructor says that this function sends the broadcast text to the intended client
    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.msg);
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
        ctx.text(msg.text);
    }
}

pub struct WebSocket {
    // Address of the websocket manager, which manages all of the websocket connections
    // it needs to implement the Actor trait so that it can be used as an Actix actor in the whole application
    pub(crate) manager: Addr<WsSessionManager>,
}

impl Actor for WebSocket {
    //defines the type of context used by the websocket actor
    type Context = ws::WebsocketContext<Self>;

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
        Running::Stop // indicates to the actor that the runtime should stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // check if the message is a valid TEXT message
        if let Ok(ws::Message::Text(text)) = msg {
            self.manager.do_send(BroadcastMessage {
                msg: text.to_string(),
                sender: ctx.address(),
            })
        }
    }
}
