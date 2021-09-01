pub trait Message: Sized {
    const NAME: &'static str;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    fn to_bytes(&self) -> Vec<u8>;
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
        if M::NAME != self.name.0 {
            return None;
        }
        M::from_bytes(&self.bytes)
    }
}

impl<M: Message> From<M> for AnyMessage {
    fn from(message: M) -> Self {
        Self {
            name: MessageName(M::NAME),
            bytes: message.to_bytes(),
        }
    }
}
