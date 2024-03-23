use crate::poker::Session;

pub trait FromSession {
    fn from_session(session: &Session) -> Self;
}

pub trait ToSession {
    fn to_session(&self) -> Session;
}
