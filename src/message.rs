use rmp_serde as rmps;
use serde::{Deserialize, Serialize};

// I think a better requirement here is for a Message to be Sendable/Receivable instead
// of Serializable/Deserializable. For the most part these are equivalent but Sendable/Receivable
// is a weaker notion as it doesn't require all the fields to be serializable.
// For example, if we deceide that a CallbackId should hold a reference to the dispatcher to allow it
// to be called directly, then it is not serializable. But it is "sendable" as the runtime should have
// enough information to "receive" it. Put it other way, being Sendable/Receivable only requires
// the fields that cannot be recovered by the runtime to be serializable.
pub trait Message: Serialize + for<'a> Deserialize<'a> {
    const NAME: &'static str;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MessageName(&'static str);

impl MessageName {
    pub fn name_for<M>() -> Self
    where
        M: Message,
    {
        MessageName(M::NAME)
    }
}

pub struct AnyMessage {
    pub name: MessageName,
    bytes: Vec<u8>,
}

impl AnyMessage {
    pub fn downcast<M>(&self) -> Option<M>
    where
        M: Message,
    {
        if self.name == MessageName::name_for::<M>() {
            // TODO: What serialization format should we use here?, it'd be nice if we can
            // guarantee absence of errors when serializing.
            rmps::from_read_ref(&self.bytes).ok()
        } else {
            None
        }
    }
}

impl<M: Message> From<M> for AnyMessage {
    fn from(message: M) -> Self {
        Self {
            name: MessageName(M::NAME),
            // TODO: we should ensure that everything that implements Message
            // is serializable (if not we should at least handle the error in Dispatcher)
            bytes: rmps::to_vec(&message).unwrap(),
        }
    }
}
