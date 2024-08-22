use crate::game;

type HeirMd = String;
impl From<HeirMd> for game::Session {
    fn from(md: HeirMd) -> Self {
        unimplemented!()
    }
}
impl Into<HeirMd> for game::Session {
    fn into(self) -> HeirMd {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let session = game::Session::exhaustive();
        let md: HeirMd = session.into();
        assert_eq!(game::Session::exhaustive(), md.into());
    }
}
