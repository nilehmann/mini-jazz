use std::marker::PhantomData;

use super::{actor::AnyActorId, message::AnyMessage};

pub struct LogEntry<L>
where
    L: Log,
{
    pub sender_id: AnyActorId,
    pub message: AnyMessage,
    pub next_idx: L::LogIndex,
    _marker: PhantomData<L>,
}

pub trait LogIndex: Copy {
    const ZERO: Self;
}

#[async_trait::async_trait]
pub trait Log: Sized {
    type LogIndex: LogIndex;
    type Error;

    async fn read(
        &mut self,
        actor_id: AnyActorId,
        idx: Self::LogIndex,
    ) -> Result<Option<LogEntry<Self>>, Self::Error>;

    async fn append(
        &mut self,
        from: AnyActorId,
        to: AnyActorId,
        msg: AnyMessage,
    ) -> Result<Self::LogIndex, Self::Error>;

    async fn trim(&self, actor_id: AnyActorId, idx: Self::LogIndex) -> Result<(), Self::Error>;
}
