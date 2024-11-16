use std::fmt;
use std::io;

/// A [`Card`] in a traditional 52-card deck.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Card {
    AceClubs = 0,
    AceDiamonds,
    AceHearts,
    AceSpades,
    TwoClubs,
    TwoDiamonds,
    TwoHearts,
    TwoSpades,
    ThreeClubs,
    ThreeDiamonds,
    ThreeHearts,
    ThreeSpades,
    FourClubs,
    FourDiamonds,
    FourHearts,
    FourSpades,
    FiveClubs,
    FiveDiamonds,
    FiveHearts,
    FiveSpades,
    SixClubs,
    SixDiamonds,
    SixHearts,
    SixSpades,
    SevenClubs,
    SevenDiamonds,
    SevenHearts,
    SevenSpades,
    EightClubs,
    EightDiamonds,
    EightHearts,
    EightSpades,
    NineClubs,
    NineDiamonds,
    NineHearts,
    NineSpades,
    TenClubs,
    TenDiamonds,
    TenHearts,
    TenSpades,
    JackClubs,
    JackDiamonds,
    JackHearts,
    JackSpades,
    QueenClubs,
    QueenDiamonds,
    QueenHearts,
    QueenSpades,
    KingClubs,
    KingDiamonds,
    KingHearts,
    KingSpades,
    Unknown,
    Xx,
}

impl Card {
    #[inline]
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    /// Convert a raw u8 into a [`Card`] enum instance.
    pub fn from_u8(value: u8) -> io::Result<Self> {
        if value <= 53 {
            // Safety: all values 0..=53 are valid [`Card`]s.
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid card value: {}.", value),
            ))
        }
    }

    #[inline]
    /// Convert a raw u8 into a [`Card`] enum instance.
    ///
    /// # Panic
    /// Out-of-bounds values will cause panic in debug mode.
    ///
    /// # Safety
    /// Callers must ensure that the value they provide map to a
    /// valid enum instance.
    pub unsafe fn from_u8_unchecked(value: u8) -> Self {
        debug_assert!(value <= 53, "Value out of bounds.");
        std::mem::transmute(value)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Card::AceClubs => write!(f, "{}", "Ac"),
            Card::AceDiamonds => write!(f, "{}", "Ad"),
            Card::AceHearts => write!(f, "{}", "Ah"),
            Card::AceSpades => write!(f, "{}", "As"),
            Card::TwoClubs => write!(f, "{}", "2c"),
            Card::TwoDiamonds => write!(f, "{}", "2d"),
            Card::TwoHearts => write!(f, "{}", "2h"),
            Card::TwoSpades => write!(f, "{}", "2s"),
            Card::ThreeClubs => write!(f, "{}", "3c"),
            Card::ThreeDiamonds => write!(f, "{}", "3d"),
            Card::ThreeHearts => write!(f, "{}", "3h"),
            Card::ThreeSpades => write!(f, "{}", "3s"),
            Card::FourClubs => write!(f, "{}", "4c"),
            Card::FourDiamonds => write!(f, "{}", "4d"),
            Card::FourHearts => write!(f, "{}", "4h"),
            Card::FourSpades => write!(f, "{}", "4s"),
            Card::FiveClubs => write!(f, "{}", "5c"),
            Card::FiveDiamonds => write!(f, "{}", "5d"),
            Card::FiveHearts => write!(f, "{}", "5h"),
            Card::FiveSpades => write!(f, "{}", "5s"),
            Card::SixClubs => write!(f, "{}", "6c"),
            Card::SixDiamonds => write!(f, "{}", "6d"),
            Card::SixHearts => write!(f, "{}", "6h"),
            Card::SixSpades => write!(f, "{}", "6s"),
            Card::SevenClubs => write!(f, "{}", "7c"),
            Card::SevenDiamonds => write!(f, "{}", "7d"),
            Card::SevenHearts => write!(f, "{}", "7h"),
            Card::SevenSpades => write!(f, "{}", "7s"),
            Card::EightClubs => write!(f, "{}", "8c"),
            Card::EightDiamonds => write!(f, "{}", "8d"),
            Card::EightHearts => write!(f, "{}", "8h"),
            Card::EightSpades => write!(f, "{}", "8s"),
            Card::NineClubs => write!(f, "{}", "9c"),
            Card::NineDiamonds => write!(f, "{}", "9d"),
            Card::NineHearts => write!(f, "{}", "9h"),
            Card::NineSpades => write!(f, "{}", "9s"),
            Card::TenClubs => write!(f, "{}", "Tc"),
            Card::TenDiamonds => write!(f, "{}", "Td"),
            Card::TenHearts => write!(f, "{}", "Th"),
            Card::TenSpades => write!(f, "{}", "Ts"),
            Card::JackClubs => write!(f, "{}", "Jc"),
            Card::JackDiamonds => write!(f, "{}", "Jd"),
            Card::JackHearts => write!(f, "{}", "Jh"),
            Card::JackSpades => write!(f, "{}", "Js"),
            Card::QueenClubs => write!(f, "{}", "Qc"),
            Card::QueenDiamonds => write!(f, "{}", "Qd"),
            Card::QueenHearts => write!(f, "{}", "Qh"),
            Card::QueenSpades => write!(f, "{}", "Qs"),
            Card::KingClubs => write!(f, "{}", "Kc"),
            Card::KingDiamonds => write!(f, "{}", "Kd"),
            Card::KingHearts => write!(f, "{}", "Kh"),
            Card::KingSpades => write!(f, "{}", "Ks"),
            Card::Unknown => write!(f, "{}", "??"),
            Card::Xx => write!(f, "{}", "Xx"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_to_u8() {
        let card = Card::AceClubs;
        assert_eq!(card.to_u8(), 0);

        let card = Card::AceSpades;
        assert_eq!(card.to_u8(), 3);

        let card = Card::KingSpades;
        assert_eq!(card.to_u8(), 51);

        let card = Card::Unknown;
        assert_eq!(card.to_u8(), 52);

        let card = Card::Xx;
        assert_eq!(card.to_u8(), 53);
    }

    #[test]
    fn test_card_from_u8_valid() {
        for value in 0..=53 {
            let card = Card::from_u8(value).expect("Valid card value");
            assert_eq!(card.to_u8(), value);
        }
    }

    #[test]
    fn test_card_from_u8_invalid() {
        let invalid_values = [54, 100, 255];
        for &value in &invalid_values {
            let result = Card::from_u8(value);
            assert!(result.is_err(), "Value {} should be invalid", value);
        }
    }

    #[test]
    fn test_card_display() {
        let card = Card::AceSpades;
        assert_eq!(format!("{}", card), "As");

        let card = Card::TwoClubs;
        assert_eq!(format!("{}", card), "2c");

        let card = Card::TenHearts;
        assert_eq!(format!("{}", card), "Th");

        let card = Card::KingDiamonds;
        assert_eq!(format!("{}", card), "Kd");

        let card = Card::Unknown;
        assert_eq!(format!("{}", card), "??");

        let card = Card::Xx;
        assert_eq!(format!("{}", card), "Xx");
    }

    #[test]
    fn test_card_from_u8_unchecked() {
        unsafe {
            for value in 0..=53 {
                let card = Card::from_u8_unchecked(value);
                assert_eq!(card.to_u8(), value);
            }
        }
    }
}
