use std::collections::HashMap;

use super::{
    actor::{AnyActorId, PersistentActorId},
    handler::Handler,
    message::{AnyMessage, Message},
};

pub struct Dispatcher {
    pub(crate) messages: HashMap<AnyActorId, AnyMessage>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    fn send<A, M>(&mut self, actor_id: PersistentActorId<A>, message: M)
    where
        M: Message,
        A: Handler<M>,
    {
        self.messages
            .insert(AnyActorId::from(actor_id), AnyMessage::from(message));
    }
}
