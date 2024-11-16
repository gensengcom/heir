use std::fmt;

/// A Session represents a collection of [`Table`]s along with some metadata.
/// Note that this struct nor its children verify the data logic, it's just a format.
/// For instance, it is possible to define a [`RakePercentage`] of 255%.
#[derive(Clone, PartialEq, Debug)]
pub struct Session {
    pub version: Version,
    pub id: Id,
    pub name: String,
    pub tables: Vec<Table>,
    pub hero_id: Id,
}

/// u32 alias for all identifiable types.
type Id = u64;

/// A Table is a continuous collection of [`Hand`]s along with an initial context and some metadata.
#[derive(Clone, PartialEq, Debug)]
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
#[derive(Clone, PartialEq, Debug)]
pub struct Player {
    pub id: Id,
    pub name: String,
    pub stack: Decimal,
}

/// An update to the state of the [`Table`].
#[derive(Clone, PartialEq, Debug)]
pub enum TableEvent {
    Hand(Hand),
    StackUpdate(StackUpdate),
    SeatUpdate(SeatUpdate),
}

/// A Hand (not pair of hole cards) that occurs at a [`Table`].
#[derive(Clone, PartialEq, Debug)]
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

/// The action of a [`Player`] at a given point in a [`Hand`].
#[derive(Clone, PartialEq, Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub bet_amount: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ActionType {
    Fold,
    Check,
    Bet,
    Call,
    Raise,
    AllIn,
}

/// An update to a [`Player`]'s stack outside of a [`Hand`] (e.g. top-up or rathole).
#[derive(Clone, PartialEq, Debug)]
pub struct StackUpdate {
    pub seat: u8,
    pub stack: u32,
}

/// An update to a [`Player`] at a [`Table`] (e.g. seat change).
#[derive(Clone, PartialEq, Debug)]
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
                        stack: 10_000,
                    },
                    Player {
                        id: 1002,
                        name: "Player 1002".to_string(),
                        stack: 10_000,
                    },
                ],
                events: vec![
                    TableEvent::Hand(Hand {
                        id: 9001,
                        button_position: 1,
                        hole_cards: vec![
                            [Card::AceClubs, Card::AceSpades],
                            [Card::TwoClubs, Card::TwoSpades],
                        ],
                        actions: vec![
                            Action {
                                action_type: ActionType::Raise,
                                bet_amount: 300,
                            },
                            Action {
                                action_type: ActionType::Call,
                                bet_amount: 300,
                            },
                            Action {
                                action_type: ActionType::Bet,
                                bet_amount: 600,
                            },
                            Action {
                                action_type: ActionType::Raise,
                                bet_amount: 1800,
                            },
                            Action {
                                action_type: ActionType::Fold,
                                bet_amount: 600,
                            },
                        ],
                        timestamp: 1724293476,
                        board: [
                            Card::ThreeClubs,
                            Card::ThreeHearts,
                            Card::KingClubs,
                            Card::Xx,
                            Card::Xx,
                        ],
                    }),
                    TableEvent::StackUpdate(StackUpdate {
                        seat: 1,
                        stack: 20_000,
                    }),
                    TableEvent::SeatUpdate(SeatUpdate {
                        seat: 0,
                        player: Some(Player {
                            id: 1003,
                            name: "Player 1003".to_string(),
                            stack: 15_000,
                        }),
                    }),
                    TableEvent::Hand(Hand {
                        id: 9002,
                        button_position: 0,
                        hole_cards: vec![
                            [Card::AceClubs, Card::AceSpades],
                            [Card::Unknown, Card::Unknown],
                        ],
                        actions: vec![
                            Action {
                                action_type: ActionType::Call,
                                bet_amount: 100,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 100,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 0,
                            },
                            Action {
                                action_type: ActionType::Bet,
                                bet_amount: 400,
                            },
                            Action {
                                action_type: ActionType::Raise,
                                bet_amount: 800,
                            },
                            Action {
                                action_type: ActionType::Raise,
                                bet_amount: 1600,
                            },
                            Action {
                                action_type: ActionType::Call,
                                bet_amount: 1600,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 0,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 0,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 0,
                            },
                            Action {
                                action_type: ActionType::Check,
                                bet_amount: 0,
                            },
                        ],
                        timestamp: 1724293500,
                        board: [
                            Card::SevenHearts,
                            Card::SevenSpades,
                            Card::SevenClubs,
                            Card::SevenDiamonds,
                            Card::EightHearts,
                        ],
                    }),
                ],
            }],
            hero_id: 0,
        }
    }
}
