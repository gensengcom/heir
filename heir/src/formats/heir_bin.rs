use crate::{poker, FromSession, ToSession};
use bincode::{deserialize, serialize};

pub struct HeirBin {
    pub data: Vec<u8>,
}

impl FromSession for HeirBin {
    fn from_session(session: &poker::Session) -> Self {
        let data = serialize(session).expect("failed to serialize session");
        Self { data }
    }
}

impl ToSession for HeirBin {
    fn to_session(&self) -> poker::Session {
        deserialize(&self.data).expect("failed to deserialize session")
    }
}
