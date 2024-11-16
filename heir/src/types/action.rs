use std::fmt;
use std::io::{self, Read, Write};

/// The action of a [`Player`] at a given point in a [`Hand`].
/// Most signifcant two bits are the [`ActionType`].
/// Least 30 bits are the bet size in cents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action(u32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
/// A 2-bit value denoting the action's type.
pub enum ActionType {
    CheckFold = 0, //00
    Call = 1,      //01
    Bet = 2,       //10
    Raise = 3,     //11
}

impl ActionType {
    /// Safely constructs a [`ActionType`] from a raw u8.
    #[inline]
    pub fn from_u8(value: u8) -> Result<Self, ActionError> {
        match value {
            0 => Ok(ActionType::CheckFold),
            1 => Ok(ActionType::Call),
            2 => Ok(ActionType::Bet),
            3 => Ok(ActionType::Raise),
            _ => Err(ActionError::InvalidU8AsActionType(value)),
        }
    }

    /// Constructs a [`ActionType`] from a raw u8.
    ///
    /// # Panic
    /// Will panic on invalid u8 in debug mode.
    ///
    /// # Safety
    /// Caller must u8 value is in range [0, 3].
    pub unsafe fn from_u8_unchecked(value: u8) -> Self {
        debug_assert!(value <= 3, "u8 action type value exceeds bounds");
        std::mem::transmute(value)
    }

    /// Converts [`ActionType`] to its `u8` representation.
    #[inline]
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl Action {
    /// Create an [`Action`] with bounds checking.
    /// Currency interpretation of `cents` is defined at the [`Table`] level.
    pub fn new(at: ActionType, cents: u32) -> Result<Self, ActionError> {
        if cents >= (1 << 30) {
            return Err(ActionError::CentsExceedsRange(cents));
        }
        let value = ((at.to_u8() as u32) << 30) | cents;
        Ok(Action(value))
    }

    /// Create an [`Action`] without bounds checking.
    /// Currency interpretation of `cents` is defined at the [`Table`] level.
    ///
    /// # Panic
    /// Panics on invalid inputs in debug mode.
    ///
    /// # Safety
    /// Caller ensures valid [`ActionType`] and cents < 2^30.
    pub unsafe fn new_unchecked(at: u8, cents: u32) -> Self {
        debug_assert!(at <= 3, "ActionType value exceeds bounds");
        debug_assert!(cents <= 0x3FFF_FFFF, "Cents value exceeds bounds");
        let value = ((at as u32) << 30) | (cents & 0x3FFF_FFFF);
        Action(value)
    }

    /// Retrieves the [`ActionType`] from the `Action`.
    pub fn action_type(&self) -> ActionType {
        let at = (self.0 >> 30) as u8;
        unsafe { ActionType::from_u8_unchecked(at) }
    }

    /// Retrieves the `cents` value from the `Action`.
    pub fn cents(&self) -> u32 {
        self.0 & 0x3FFF_FFFF
    }

    /// Serializes the `Action` into the given writer.
    pub fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0.to_le_bytes())
    }

    /// Deserializes an `Action` from the given reader.
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Self, ActionError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let value = u32::from_le_bytes(buf);
        let at_u8 = (value >> 30) as u8;
        let _at = ActionType::from_u8(at_u8)?;
        let _cents = value & 0x3FFF_FFFF;
        Ok(Action(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    InvalidU8AsActionType(u8),
    CentsExceedsRange(u32),
    IoError(io::ErrorKind),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::InvalidU8AsActionType(u8) => {
                write!(f, "Invalid u8 {} as ActionType; must be in 0..=3.", u8)
            }
            ActionError::CentsExceedsRange(u32) => {
                write!(f, "Cents value {} is exceeds 2^30 - 1.", u32)
            }
            ActionError::IoError(kind) => {
                write!(
                    f,
                    "IO error during serialization/deserialization: {:?}",
                    kind
                )
            }
        }
    }
}

impl std::error::Error for ActionError {}

impl From<io::Error> for ActionError {
    fn from(err: io::Error) -> Self {
        ActionError::IoError(err.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_new_valid() {
        let action = Action::new(ActionType::Call, 100).expect("Valid action");
        assert_eq!(action.action_type(), ActionType::Call);
        assert_eq!(action.cents(), 100);

        let action = Action::new(ActionType::Raise, (1 << 30) - 1).expect("Valid action");
        assert_eq!(action.action_type(), ActionType::Raise);
        assert_eq!(action.cents(), (1 << 30) - 1);
    }

    #[test]
    fn test_action_new_invalid_cents() {
        let result = Action::new(ActionType::Bet, 1 << 30);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, ActionError::CentsExceedsRange(1 << 30));
        }
    }

    #[test]
    fn test_actiontype_from_u8_valid() {
        for value in 0..=3 {
            let at = ActionType::from_u8(value).expect("Valid ActionType");
            assert_eq!(at.to_u8(), value);
        }
    }

    #[test]
    fn test_actiontype_from_u8_invalid() {
        let invalid_values = [4, 5, 255];
        for &value in &invalid_values {
            let result = ActionType::from_u8(value);
            assert!(result.is_err(), "Value {} should be invalid", value);
        }
    }

    #[test]
    fn test_action_new_unchecked() {
        unsafe {
            let action = Action::new_unchecked(1, 200);
            assert_eq!(action.action_type(), ActionType::Call);
            assert_eq!(action.cents(), 200);
        }
    }

    #[test]
    fn test_action_action_type_and_cents() {
        let action = Action::new(ActionType::Bet, 500).expect("Valid action");
        assert_eq!(action.action_type(), ActionType::Bet);
        assert_eq!(action.cents(), 500);
    }

    #[test]
    fn test_action_serialize_deserialize() {
        let original_action = Action::new(ActionType::Raise, 1000).expect("Valid action");
        let mut buffer = Vec::new();
        original_action
            .serialize(&mut buffer)
            .expect("Serialization failed");

        let mut cursor = &buffer[..];
        let deserialized_action = Action::deserialize(&mut cursor).expect("Deserialization failed");

        assert_eq!(original_action, deserialized_action);
        assert_eq!(deserialized_action.action_type(), ActionType::Raise);
        assert_eq!(deserialized_action.cents(), 1000);
    }

    #[test]
    fn test_action_deserialize_invalid_action_type() {
        // Create invalid data with an invalid ActionType (e.g., 4)
        let invalid_value = (4u32 << 30) | 500;
        let buffer = invalid_value.to_le_bytes().to_vec();

        let mut cursor = &buffer[..];
        let result = Action::deserialize(&mut cursor);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, ActionError::InvalidU8AsActionType(4)));
        }
    }

    #[test]
    fn test_action_deserialize_io_error() {
        // Provide insufficient data to trigger an IO error
        let buffer = vec![0u8; 3]; // Should be 4 bytes
        let mut cursor = &buffer[..];
        let result = Action::deserialize(&mut cursor);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e, ActionError::IoError(io::ErrorKind::UnexpectedEof));
        }
    }
}
