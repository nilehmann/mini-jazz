use super::{actor::PersistentActor, context::Context, message::Message};

pub trait Handler<M: Message>: PersistentActor {
    fn handle(&self, cx: &mut Context<Self>, message: M);
}
