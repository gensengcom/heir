pub struct Session {
    pub id: u32,
    pub tables: Vec<Table>,
    pub hero_id: u32,
}

pub struct Table {
    pub id: u32,
    pub location: String,
    pub table_size: u8,
    pub rake_percentage: u8,
    pub rake_cap: u32,
    pub blinds: Vec<u32>,
    pub initial_context: Vec<Player>,
    pub events: Vec<Event>,
}

pub struct Player {
    pub id: u32,
    pub stack: u32,
}

pub enum Event {
    Hand(Hand),
    StackUpdate(StackUpdate),
    SeatUpdate(SeatUpdate),
}

pub struct Hand {
    pub id: u32,
    pub button_position: u8,
    pub hole_cards: Vec<[u8; 2]>,
    pub actions: Vec<Action>,
    pub timestamp: u64,
    pub board: u32,
}

pub struct Action {
    pub player_and_action: u8,
    pub bet_amount: u32,
}

pub struct StackUpdate {
    pub seat: u8,
    pub stack: u32,
}

pub struct SeatUpdate {
    pub seat: u8,
    pub player: Option<Player>,
}
