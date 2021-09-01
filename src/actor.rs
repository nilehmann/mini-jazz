use std::{marker::PhantomData, panic::RefUnwindSafe};

use super::context::Context;

pub trait PersistentActor: 'static + Sized + RefUnwindSafe {
    const NAME: &'static str;

    fn init(&self, cx: &mut Context<Self>) {}
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

impl<A> Clone for PersistentActorId<A>
where
    A: PersistentActor,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _marker: PhantomData,
        }
    }
}

impl<A> Copy for PersistentActorId<A> where A: PersistentActor {}

impl AnyActorId {
    pub fn downcast<A>(&self) -> Option<PersistentActorId<A>>
    where
        A: PersistentActor,
    {
        if self.name != ActorName::name_for::<A>() {
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
