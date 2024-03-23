use crate::model::Session;

pub trait FromSession {
    fn from_session(session: &Session) -> Self;
}

pub trait ToSession {
    fn to_session(&self) -> Session;
}
