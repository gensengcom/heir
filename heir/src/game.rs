use bincode::{Decode, Encode};

/// A Session represents a collection of [`Table`]s along with some metadata.
/// Note that this struct nor its children verify the data logic, it's just a format.
/// For instance, it is possible to define a [`RakePercentage`] of 255%.
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct Session {
    pub version: Version,
    pub id: Id,
    pub name: String,
    pub tables: Vec<Table>,
    pub hero_id: Id,
}

/// An alias for a semver String.
type Version = String;

/// u32 alias for all identifiable types.
type Id = u64;

/// A Table is a continuous collection of [`Hand`]s along with an initial context and some metadata.
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct Table {
    pub id: Id,
    pub name: String,
    pub location: String,
    pub table_size: TableSize,
    pub rake_percentage: RakePercentage,
    pub rake_cap: RakeCap,
    pub blinds: Vec<Decimal>,
    pub initial_context: Vec<Player>,
    pub events: Vec<TableEvent>,
}

/// An alias for the number of seats at a [`Table`] (e.g. 6-max, 9-max).
type TableSize = u8;

/// The rake (generally at a given [`Table`]) as a percentage 0-100.
type RakePercentage = u8;

/// The Table's rake cap as [`Decimal`].
type RakeCap = Decimal;

/// A Decimal is a u64 representing a number of cents.
type Decimal = u64;

/// A context for a player in a seat at a [`Table`].
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct Player {
    pub id: Id,
    pub name: String,
    pub stack: Decimal,
}

/// An update to the state of the [`Table`].
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum TableEvent {
    Hand(Hand),
    StackUpdate(StackUpdate),
    SeatUpdate(SeatUpdate),
}

/// A Hand (not pair of hole cards) that occurs at a [`Table`].
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct Hand {
    pub id: Id,
    pub button_position: ButtonPosition,
    pub hole_cards: HoleCards,
    pub actions: Vec<Action>,
    pub timestamp: Timestamp,
    pub board: Board,
}

/// A Vec hodling two [`Card`]s for each [`Player`] with a known starting [`Hand`].
type HoleCards = Vec<[Card; 2]>;

/// A u64 representing a UNIX timestamp.
type Timestamp = u64;

/// The position of the button at a [`Table`] during a given [`Hand`].
type ButtonPosition = u8;

/// 0-5 [`Card`]s on the board.
type Board = [Card; 5];

/// A card is represented as a u8.
#[repr(u8)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
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
}

impl Card {
    pub fn to_string(&self) -> String {
        match self {
            Card::AceClubs => "Ac".to_string(),
            Card::AceDiamonds => "Ad".to_string(),
            Card::AceHearts => "Ah".to_string(),
            Card::AceSpades => "As".to_string(),
            Card::TwoClubs => "2c".to_string(),
            Card::TwoDiamonds => "2d".to_string(),
            Card::TwoHearts => "2h".to_string(),
            Card::TwoSpades => "2s".to_string(),
            Card::ThreeClubs => "3c".to_string(),
            Card::ThreeDiamonds => "3d".to_string(),
            Card::ThreeHearts => "3h".to_string(),
            Card::ThreeSpades => "3s".to_string(),
            Card::FourClubs => "4c".to_string(),
            Card::FourDiamonds => "4d".to_string(),
            Card::FourHearts => "4h".to_string(),
            Card::FourSpades => "4s".to_string(),
            Card::FiveClubs => "5c".to_string(),
            Card::FiveDiamonds => "5d".to_string(),
            Card::FiveHearts => "5h".to_string(),
            Card::FiveSpades => "5s".to_string(),
            Card::SixClubs => "6c".to_string(),
            Card::SixDiamonds => "6d".to_string(),
            Card::SixHearts => "6h".to_string(),
            Card::SixSpades => "6s".to_string(),
            Card::SevenClubs => "7c".to_string(),
            Card::SevenDiamonds => "7d".to_string(),
            Card::SevenHearts => "7h".to_string(),
            Card::SevenSpades => "7s".to_string(),
            Card::EightClubs => "8c".to_string(),
            Card::EightDiamonds => "8d".to_string(),
            Card::EightHearts => "8h".to_string(),
            Card::EightSpades => "8s".to_string(),
            Card::NineClubs => "9c".to_string(),
            Card::NineDiamonds => "9d".to_string(),
            Card::NineHearts => "9h".to_string(),
            Card::NineSpades => "9s".to_string(),
            Card::TenClubs => "Tc".to_string(),
            Card::TenDiamonds => "Td".to_string(),
            Card::TenHearts => "Th".to_string(),
            Card::TenSpades => "Ts".to_string(),
            Card::JackClubs => "Jc".to_string(),
            Card::JackDiamonds => "Jd".to_string(),
            Card::JackHearts => "Jh".to_string(),
            Card::JackSpades => "Js".to_string(),
            Card::QueenClubs => "Qc".to_string(),
            Card::QueenDiamonds => "Qd".to_string(),
            Card::QueenHearts => "Qh".to_string(),
            Card::QueenSpades => "Qs".to_string(),
            Card::KingClubs => "Kc".to_string(),
            Card::KingDiamonds => "Kd".to_string(),
            Card::KingHearts => "Kh".to_string(),
            Card::KingSpades => "Ks".to_string(),
        }
    }
}

/// The action of a [`Player`] at a given point in a [`Hand`].
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct Action {
    pub player_and_action: u8,
    pub bet_amount: u32,
}

/// An update to a [`Player`]'s stack outside of a [`Hand`] (e.g. top-up or rathole).
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct StackUpdate {
    pub seat: u8,
    pub stack: u32,
}

/// An update to a [`Player`] at a [`Table`] (e.g. seat change).
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct SeatUpdate {
    pub seat: u8,
    pub player: Option<Player>,
}

#[cfg(test)]
impl Session {
    /// Returns an [`Session`] that covers all possible [`Table`] entries for testing purposes.
    pub fn exhaustive() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            id: 1738,
            name: "Exhaustive Session".to_string(),
            tables: vec![Table {
                id: 1,
                name: "Table 1".to_string(),
                location: "North Avenue East 1205".to_string(),
                table_size: 2,
                rake_percentage: 5,
                rake_cap: 3,
                blinds: vec![50, 100],
                initial_context: vec![
                    Player {
                        id: 1001,
                        name: "Player 1001".to_string(),
                        stack: 100_00,
                    },
                    Player {
                        id: 1002,
                        name: "Player 1002".to_string(),
                        stack: 100_00,
                    },
                ],
                events: vec![TableEvent::Hand {
                    id: todo!(),
                    button_position: todo!(),
                    hole_cards: todo!(),
                    actions: todo!(),
                    timestamp: todo!(),
                    board: todo!(),
                }],
            }],
            hero_id: 2,
        }
    }
}
