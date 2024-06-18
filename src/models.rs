use actix::Message;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct MyWebSocketMessage(pub String);
