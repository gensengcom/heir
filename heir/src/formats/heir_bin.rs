use crate::game;

use bincode::{config, config::Configuration};

/// The global config for consistent use with the [`bincode`] crate.
const BINCODE_CONFIG: Configuration = config::standard();

type HeirBin = Vec<u8>;
impl From<HeirBin> for game::Session {
    fn from(bin: HeirBin) -> Self {
        let (decoded, _len): (Self, usize) =
            bincode::decode_from_slice(&bin, BINCODE_CONFIG).unwrap();
        decoded
    }
}
impl Into<HeirBin> for game::Session {
    fn into(self) -> HeirBin {
        bincode::encode_to_vec(self, BINCODE_CONFIG).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let bin: HeirBin = game::Session::exhaustive().into();
        assert_eq!(game::Session::exhaustive(), bin.into());
    }
}
