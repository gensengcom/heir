use crate::types::card::Card;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board(u32);

impl Board {
    pub fn new() -> Self {
        let xx = Card::Xx.to_u8() as u32;
        let packed = (xx << 24) | (xx << 18) | (xx << 12) | (xx << 6) | xx;
        Board(packed)
    }

    /// Set a card at an index within the board.
    #[inline]
    pub fn set_card(&mut self, index: usize, card: Card) -> io::Result<()> {
        if index >= 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds, boards contain 5 cards.",
            ));
        }
        // 0x3F mask preserves rightmost 6 bits
        let card_val = (card.to_u8() as u32) & 0x3F;
        let shift = index * 6;
        self.0 &= !(0x3F << shift);
        self.0 |= card_val << shift;
        Ok(())
    }

    /// Set a card at an index within the board.
    ///
    /// # Panic
    /// Out-of-bounds incides will panic in debug mode.
    ///
    /// # Safety
    /// Caller must ensure that the index is less than 5.
    #[inline]
    pub unsafe fn set_card_unchecked(&mut self, index: usize, card: Card) {
        debug_assert!(index < 5, "Index out of bounds");

        // 0x3F mask preserves rightmost 6 bits
        let card_value = (card.to_u8() as u32) & 0x3F; // Mask to 6 bits
        let shift = index * 6;
        self.0 &= !(0x3F << shift); // Clear the bits at the position
        self.0 |= card_value << shift; // Set the new card value
    }

    /// Gets the card at the given index (0 to 4).
    #[inline]
    pub fn get_card(&self, index: usize) -> io::Result<Card> {
        if index >= 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ));
        }
        let shift = index * 6;
        let card_value = ((self.0 >> shift) & 0x3F) as u8;
        Card::from_u8(card_value)
    }

    /// Unsafely gets the card at the given index (0 to 4).
    ///
    /// # Panics
    /// Panics in debug builds if `index` is out of bounds (not in 0..5).
    ///
    /// # Safety
    /// Assumes that the internal data of `Board` is valid. This is guaranteed
    /// if `Board` is only modified using `set_card` and data is validated during deserialization.
    #[inline]
    pub unsafe fn get_card_unchecked(&self, index: usize) -> Card {
        debug_assert!(index < 5, "Index out of bounds");
        let shift = index * 6;
        let card_val = ((self.0 >> shift) & 0x3F) as u8;
        unsafe { Card::from_u8_unchecked(card_val) }
    }

    /// Convert a [`Board`] to an array of five [`Card`]s.
    pub fn to_array(&self) -> io::Result<[Card; 5]> {
        let mut cards = [Card::Xx; 5];
        for i in 0..5 {
            cards[i] = self.get_card(i)?;
        }
        Ok(cards)
    }

    /// Convert an array of [`Card`]s to a [`Board`].
    pub fn from_array(cards: [Card; 5]) -> Self {
        let mut board = Board(0);
        for (i, card) in cards.iter().enumerate() {
            // Safety: index < 5 implied by static `cards` length
            unsafe { board.set_card_unchecked(i, *card) };
        }
        board
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.0.to_le_bytes())
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let packed = u32::from_le_bytes(buf);
        let board = Board(packed);

        for i in 0..5 {
            board.get_card(i)?;
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::card::Card;

    #[test]
    fn test_board_packing() -> io::Result<()> {
        let cards = [
            Card::AceSpades,
            Card::KingHearts,
            Card::TenClubs,
            Card::FiveDiamonds,
            Card::TwoHearts,
        ];
        let board = Board::from_array(cards);
        let retrieved_cards = board.to_array()?;
        assert_eq!(cards, retrieved_cards);
        Ok(())
    }

    #[test]
    fn test_board_serialization() -> io::Result<()> {
        let cards = [
            Card::AceSpades,
            Card::KingHearts,
            Card::TenClubs,
            Card::FiveDiamonds,
            Card::TwoHearts,
        ];
        let board = Board::from_array(cards);

        let mut buffer = Vec::new();
        board.serialize(&mut buffer)?;

        let mut reader = &buffer[..];
        let deserialized_board = Board::deserialize(&mut reader)?;

        assert_eq!(board, deserialized_board);
        Ok(())
    }

    #[test]
    fn test_set_and_get_card() -> io::Result<()> {
        let mut board = Board::new();
        board.set_card(2, Card::QueenDiamonds)?;
        let card = board.get_card(2)?;
        assert_eq!(card, Card::QueenDiamonds);
        Ok(())
    }

    #[test]
    fn test_invalid_card_index() {
        let mut board = Board::new();
        assert!(board.set_card(5, Card::AceClubs).is_err());
        assert!(board.get_card(5).is_err());
    }
}
