use std::{marker::PhantomData, panic::RefUnwindSafe};

use super::context::Context;

pub trait PersistentActor: 'static + RefUnwindSafe {
    const NAME: &'static str;

    fn init(&self, cx: &mut Context);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ActorName(&'static str);

pub struct PersistentActorId<T>
where
    T: PersistentActor,
{
    value: u32,
    _marker: PhantomData<T>,
}

impl<T> PersistentActorId<T>
where
    T: PersistentActor,
{
    pub fn into_any(&self) -> AnyActorId {
        AnyActorId {
            name: ActorName(T::NAME),
            value: self.value,
        }
    }
}

impl ActorName {
    pub fn name_for<A>() -> Self
    where
        A: PersistentActor,
    {
        ActorName(A::NAME)
    }
}

impl<T> PersistentActorId<T>
where
    T: PersistentActor,
{
    pub(crate) fn new(value: u32) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct AnyActorId {
    pub name: ActorName,
    value: u32,
}

impl AnyActorId {
    pub fn downcast<A>(&self) -> Option<PersistentActorId<A>>
    where
        A: PersistentActor,
    {
        if self.name.0 != A::NAME {
            None
        } else {
            Some(PersistentActorId {
                value: self.value,
                _marker: PhantomData,
            })
        }
    }
}

impl<A> From<PersistentActorId<A>> for AnyActorId
where
    A: PersistentActor,
{
    fn from(actor_id: PersistentActorId<A>) -> Self {
        actor_id.into_any()
    }
}
