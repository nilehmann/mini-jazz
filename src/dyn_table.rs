use std::{any::Any, collections::HashMap, panic::RefUnwindSafe};

use crate::{
    actor::{ActorName, PersistentActor},
    context::{AnyContext, Effects},
    handler::Handler,
    message::{AnyMessage, Message, MessageName},
};

pub type AnyHandler =
    dyn Fn(&dyn Any, AnyContext, AnyMessage) -> DispatchResult<Effects> + RefUnwindSafe;
pub type AnyInit = dyn Fn(&dyn Any, AnyContext) -> DispatchResult<Effects> + RefUnwindSafe;

#[derive(PartialEq, Eq, Hash)]
struct HandlerId(ActorName, MessageName);

pub struct DynTable {
    handlers: HashMap<HandlerId, Box<AnyHandler>>,
    init: HashMap<ActorName, Box<AnyInit>>,
}

pub type DispatchResult<T> = Result<T, DispatchError>;

pub enum DispatchError {
    TypeMissmatch,
    MethodNotFound,
}

impl DynTable {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            init: HashMap::new(),
        }
    }

    pub fn is_registered<A>(&self) -> bool
    where
        A: PersistentActor,
    {
        self.init.contains_key(&ActorName::name_for::<A>())
    }

    pub fn register_handler<A, M>(&mut self)
    where
        M: Message,
        A: Handler<M>,
    {
        let handler =
            |actor: &dyn Any, cx: AnyContext, message: AnyMessage| -> DispatchResult<Effects> {
                let actor = actor
                    .downcast_ref::<A>()
                    .ok_or(DispatchError::TypeMissmatch)?;
                let mut cx = cx.downcast::<A>().ok_or(DispatchError::TypeMissmatch)?;
                let message = message
                    .downcast::<M>()
                    .ok_or(DispatchError::TypeMissmatch)?;
                actor.handle(&mut cx, message);
                Ok(cx.into_effects())
            };
        let handler_id = HandlerId(ActorName::name_for::<A>(), MessageName::name_for::<M>());
        if self.handlers.contains_key(&handler_id) {
            panic!("Handler already exists")
        }
        self.handlers.insert(handler_id, Box::new(handler));
    }

    pub fn register_actor<A>(&mut self)
    where
        A: PersistentActor,
    {
        let init = |actor: &dyn Any, cx: AnyContext| -> DispatchResult<Effects> {
            let actor = actor
                .downcast_ref::<A>()
                .ok_or(DispatchError::TypeMissmatch)?;
            let mut cx = cx.downcast::<A>().ok_or(DispatchError::TypeMissmatch)?;
            actor.init(&mut cx);
            Ok(cx.into_effects())
        };
        let actor_name = ActorName::name_for::<A>();
        if self.init.contains_key(&actor_name) {
            panic!("Init method already registered");
        }
        self.init.insert(actor_name, Box::new(init));
    }

    pub fn dispatch_init(
        &self,
        actor_name: ActorName,
        actor: &dyn Any,
        cx: AnyContext,
    ) -> DispatchResult<Effects> {
        let init = self
            .init
            .get(&actor_name)
            .ok_or(DispatchError::MethodNotFound)?;
        init(actor, cx)
    }

    pub fn dispatch_handler(
        &self,
        actor_name: ActorName,
        actor: &dyn Any,
        cx: AnyContext,
        message: AnyMessage,
    ) -> DispatchResult<Effects> {
        let handler_id = HandlerId(actor_name, message.name);
        let handler = self
            .handlers
            .get(&handler_id)
            .ok_or(DispatchError::MethodNotFound)?;
        handler(actor, cx, message)
    }
}
