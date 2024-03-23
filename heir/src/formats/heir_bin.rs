use std::fs::File;
use std::io::{BufWriter, Read, Write};

const MAGIC_NUMBER: [u8; 4] = [0x48, 0x45, 0x49, 0x52]; // "HEIR"
const VERSION: u8 = 0x00;

pub struct HeirBin {
    session_id: u32,
    tables: Vec<Table>,
}

struct Table {
    id: u32,
    location: String,
    table_size: u8,
    initial_context: Vec<Player>,
    events: Vec<Event>,
}

#[derive(Debug)]
struct Player {
    id: u32,
    stack: u32,
}

enum Event {
    Hand(Hand),
    StackUpdate(StackUpdate),
    SeatUpdate(SeatUpdate),
}

struct Hand {
    id: u32,
    button_position: u8,
    hole_cards: Vec<[u8; 2]>,
    actions: Vec<Action>,
    timestamp: u64,
    board: u32, //2 empty bits, then 5 cards
}

struct Action {
    player_and_action: u8,
    bet_amount: u32,
}

struct StackUpdate {
    seat: u8,
    stack: u32,
}

struct SeatUpdate {
    seat: u8,
    player: Option<Player>,
}

impl HeirBin {
    pub fn write_to_file<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&MAGIC_NUMBER)?;
        writer.write_all(&[VERSION])?;
        writer.write_all(&self.session_id.to_le_bytes())?;
        writer.write_all(&[self.tables.len() as u8])?;

        for table in &self.tables {
            writer.write_all(&table.id.to_le_bytes())?;
            writer.write_all(table.location.as_bytes())?;
            writer.write_all(&[0])?; // Null-terminator for location
            writer.write_all(&[table.table_size])?;
            writer.write_all(&[table.initial_context.len() as u8])?;
            for player in &table.initial_context {
                writer.write_all(&player.id.to_le_bytes())?;
                writer.write_all(&player.stack.to_le_bytes())?;
            }
            writer.write_all(&(table.events.len() as u32).to_le_bytes())?;
            for event in &table.events {
                match event {
                    Event::Hand(hand) => {
                        writer.write_all(&[0])?; // Event type: hand
                        writer.write_all(&hand.id.to_le_bytes())?;
                        writer.write_all(&[hand.button_position])?;
                        writer.write_all(&[hand.hole_cards.len() as u8])?;
                        for hole_cards in &hand.hole_cards {
                            writer.write_all(hole_cards)?;
                        }
                        writer.write_all(&[hand.actions.len() as u8])?;
                        for action in &hand.actions {
                            writer.write_all(&[action.player_and_action])?;
                            writer.write_all(&action.bet_amount.to_le_bytes())?;
                        }
                        writer.write_all(&hand.timestamp.to_le_bytes())?;
                        writer.write_all(&hand.board.to_le_bytes())?;
                    }
                    Event::StackUpdate(update) => {
                        writer.write_all(&[1])?; // Event type: stack update
                        writer.write_all(&[update.seat])?;
                        writer.write_all(&update.stack.to_le_bytes())?;
                    }
                    Event::SeatUpdate(update) => {
                        writer.write_all(&[2])?; // Event type: seat update
                        writer.write_all(&[update.seat])?;
                        match &update.player {
                            Some(player) => {
                                writer.write_all(&[1])?; // Player present
                                writer.write_all(&player.id.to_le_bytes())?;
                                writer.write_all(&player.stack.to_le_bytes())?;
                            }
                            None => {
                                writer.write_all(&[0])?; // Player absent
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn read_from_file<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if magic_number != MAGIC_NUMBER {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid magic number",
            ));
        }

        let mut version = [0u8; 1];
        reader.read_exact(&mut version)?;
        if version[0] != VERSION {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unsupported version",
            ));
        }

        let mut session_id_bytes = [0u8; 4];
        reader.read_exact(&mut session_id_bytes)?;
        let session_id = u32::from_le_bytes(session_id_bytes);

        let mut num_tables_bytes = [0u8; 1];
        reader.read_exact(&mut num_tables_bytes)?;
        let num_tables = num_tables_bytes[0];

        let mut tables = Vec::new();
        for _ in 0..num_tables {
            let mut table_id_bytes = [0u8; 4];
            reader.read_exact(&mut table_id_bytes)?;
            let table_id = u32::from_le_bytes(table_id_bytes);

            let mut location = Vec::new();
            loop {
                let mut byte = [0u8; 1];
                reader.read_exact(&mut byte)?;
                if byte[0] == 0 {
                    break;
                }
                location.push(byte[0]);
            }
            let location = String::from_utf8(location).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid location string")
            })?;

            let mut table_size = [0u8; 1];
            reader.read_exact(&mut table_size)?;

            let mut num_players = [0u8; 1];
            reader.read_exact(&mut num_players)?;

            let mut initial_context = Vec::new();
            for _ in 0..num_players[0] {
                let mut player_id_bytes = [0u8; 4];
                reader.read_exact(&mut player_id_bytes)?;
                let player_id = u32::from_le_bytes(player_id_bytes);

                let mut stack_bytes = [0u8; 4];
                reader.read_exact(&mut stack_bytes)?;
                let stack = u32::from_le_bytes(stack_bytes);

                initial_context.push(Player {
                    id: player_id,
                    stack,
                });
            }

            let mut num_events_bytes = [0u8; 4];
            reader.read_exact(&mut num_events_bytes)?;
            let num_events = u32::from_le_bytes(num_events_bytes);

            let mut events = Vec::new();
            for _ in 0..num_events {
                let mut event_type_byte = [0u8; 1];
                reader.read_exact(&mut event_type_byte)?;
                match event_type_byte[0] {
                    0 => {
                        let mut hand_id_bytes = [0u8; 4];
                        reader.read_exact(&mut hand_id_bytes)?;
                        let hand_id = u32::from_le_bytes(hand_id_bytes);

                        let mut button_position = [0u8; 1];
                        reader.read_exact(&mut button_position)?;

                        let mut num_players = [0u8; 1];
                        reader.read_exact(&mut num_players)?;

                        let mut hole_cards = Vec::new();
                        for _ in 0..num_players[0] {
                            let mut card = [0u8; 2];
                            reader.read_exact(&mut card)?;
                            hole_cards.push(card);
                        }

                        let mut num_actions_bytes = [0u8; 1];
                        reader.read_exact(&mut num_actions_bytes)?;
                        let num_actions = num_actions_bytes[0];

                        let mut actions = Vec::new();
                        for _ in 0..num_actions {
                            let mut player_and_action = [0u8; 1];
                            reader.read_exact(&mut player_and_action)?;

                            let mut bet_amount_bytes = [0u8; 4];
                            reader.read_exact(&mut bet_amount_bytes)?;
                            let bet_amount = u32::from_le_bytes(bet_amount_bytes);

                            actions.push(Action {
                                player_and_action: player_and_action[0],
                                bet_amount,
                            });
                        }

                        let mut timestamp_bytes = [0u8; 8];
                        reader.read_exact(&mut timestamp_bytes)?;
                        let timestamp = u64::from_le_bytes(timestamp_bytes);

                        let mut board_bytes = [0u8; 4];
                        reader.read_exact(&mut board_bytes)?;
                        let board = u32::from_le_bytes(board_bytes);

                        events.push(Event::Hand(Hand {
                            id: hand_id,
                            button_position: button_position[0],
                            hole_cards,
                            actions,
                            timestamp,
                            board,
                        }));
                    }
                    1 => {
                        let mut seat = [0u8; 1];
                        reader.read_exact(&mut seat)?;

                        let mut stack_bytes = [0u8; 4];
                        reader.read_exact(&mut stack_bytes)?;
                        let stack = u32::from_le_bytes(stack_bytes);

                        events.push(Event::StackUpdate(StackUpdate {
                            seat: seat[0],
                            stack,
                        }));
                    }
                    2 => {
                        let mut seat = [0u8; 1];
                        reader.read_exact(&mut seat)?;

                        let mut player_present = [0u8; 1];
                        reader.read_exact(&mut player_present)?;

                        let player = match player_present[0] {
                            1 => {
                                let mut player_id_bytes = [0u8; 4];
                                reader.read_exact(&mut player_id_bytes)?;
                                let player_id = u32::from_le_bytes(player_id_bytes);

                                let mut stack_bytes = [0u8; 4];
                                reader.read_exact(&mut stack_bytes)?;
                                let stack = u32::from_le_bytes(stack_bytes);

                                Some(Player {
                                    id: player_id,
                                    stack,
                                })
                            }
                            0 => None,
                            _ => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Invalid player presence byte",
                                ))
                            }
                        };

                        events.push(Event::SeatUpdate(SeatUpdate {
                            seat: seat[0],
                            player,
                        }));
                    }
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid event type",
                        ))
                    }
                }
            }

            tables.push(Table {
                id: table_id,
                location,
                table_size: table_size[0],
                initial_context,
                events,
            });
        }

        Ok(Self { session_id, tables })
    }

    pub fn transpile_to_heir(&self, output_path: &str) -> std::io::Result<()> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "Session ID: {}", self.session_id)?;

        for table in &self.tables {
            writeln!(writer, "Table ID: {}", table.id)?;
            writeln!(writer, "Location: {}", table.location)?;
            writeln!(writer, "Table Size: {}", table.table_size)?;

            writeln!(writer, "Initial Context:")?;
            for player in &table.initial_context {
                writeln!(
                    writer,
                    "  Player ID: {}, Stack: {}",
                    player.id, player.stack
                )?;
            }

            writeln!(writer, "Events:")?;
            for event in &table.events {
                match event {
                    Event::Hand(hand) => {
                        writeln!(writer, "  Hand:")?;
                        writeln!(writer, "    ID: {}", hand.id)?;
                        writeln!(writer, "    Button Position: {}", hand.button_position)?;
                        writeln!(writer, "    Hole Cards:")?;
                        for cards in &hand.hole_cards {
                            writeln!(writer, "      - {:#04x}{:#04x}", cards[0], cards[1])?;
                        }
                        writeln!(writer, "    Actions:")?;
                        for action in &hand.actions {
                            let player_index = action.player_and_action >> 4;
                            let action_type = action.player_and_action & 0x0F;
                            writeln!(
                                writer,
                                "      - Player: {}, Action: {}, Bet: {}",
                                player_index, action_type, action.bet_amount
                            )?;
                        }
                        writeln!(writer, "    Timestamp: {}", hand.timestamp)?;
                        let board_cards = unpack_board(hand.board);
                        writeln!(
                            writer,
                            "    Board: {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}",
                            board_cards[0],
                            board_cards[1],
                            board_cards[2],
                            board_cards[3],
                            board_cards[4]
                        )?;
                    }
                    Event::StackUpdate(update) => {
                        writeln!(writer, "  Stack Update:")?;
                        writeln!(writer, "    Seat: {}", update.seat)?;
                        writeln!(writer, "    Stack: {}", update.stack)?;
                    }
                    Event::SeatUpdate(update) => {
                        writeln!(writer, "  Seat Update:")?;
                        writeln!(writer, "    Seat: {}", update.seat)?;
                        match &update.player {
                            Some(player) => {
                                writeln!(writer, "    Player ID: {}", player.id)?;
                                writeln!(writer, "    Stack: {}", player.stack)?;
                            }
                            None => {
                                writeln!(writer, "    Player Left")?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn unpack_board(packed_board: u32) -> [u8; 5] {
    let mut board = [0u8; 5];
    for i in 0..5 {
        board[i] = ((packed_board >> (i * 6)) & 0x3F) as u8;
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Seek, SeekFrom};

    #[test]
    fn test_heir_bin_file() {
        // create sample file
        let file = HeirBin {
            session_id: 1,
            tables: vec![Table {
                id: 1,
                location: "Casino1".to_string(),
                table_size: 6,
                initial_context: vec![
                    Player {
                        id: 1,
                        stack: 10000,
                    },
                    Player { id: 2, stack: 5000 },
                ],
                events: vec![
                    Event::Hand(Hand {
                        id: 1,
                        button_position: 0,
                        hole_cards: vec![[0x2C, 0x3D], [0x44, 0x55]],
                        actions: vec![
                            Action {
                                player_and_action: pack_player_and_action(0, 1),
                                bet_amount: 100,
                            },
                            Action {
                                player_and_action: pack_player_and_action(1, 2),
                                bet_amount: 200,
                            },
                        ],
                        timestamp: 1621234567,
                        board: pack_board(&[0x6D, 0x7C, 0x88, 0x99, 0xAA]),
                    }),
                    Event::StackUpdate(StackUpdate {
                        seat: 0,
                        stack: 9900,
                    }),
                    Event::SeatUpdate(SeatUpdate {
                        seat: 2,
                        player: Some(Player { id: 3, stack: 7500 }),
                    }),
                ],
            }],
        };

        let mut buffer = Vec::new();
        file.write_to_file(&mut buffer).unwrap();

        let mut reader = Cursor::new(buffer);
        let read_file = HeirBin::read_from_file(&mut reader).unwrap();

        assert_eq!(read_file.session_id, file.session_id);
        assert_eq!(read_file.tables.len(), file.tables.len());

        let read_table = &read_file.tables[0];
        let original_table = &file.tables[0];

        assert_eq!(read_table.id, original_table.id);
        assert_eq!(read_table.location, original_table.location);
        assert_eq!(read_table.table_size, original_table.table_size);

        assert_eq!(
            read_table.initial_context.len(),
            original_table.initial_context.len()
        );
        for (read_player, original_player) in read_table
            .initial_context
            .iter()
            .zip(original_table.initial_context.iter())
        {
            assert_eq!(read_player.id, original_player.id);
            assert_eq!(read_player.stack, original_player.stack);
        }

        assert_eq!(read_table.events.len(), original_table.events.len());

        for (read_event, original_event) in
            read_table.events.iter().zip(original_table.events.iter())
        {
            match (read_event, original_event) {
                (Event::Hand(read_hand), Event::Hand(original_hand)) => {
                    assert_eq!(read_hand.id, original_hand.id);
                    assert_eq!(read_hand.button_position, original_hand.button_position);
                    assert_eq!(read_hand.hole_cards, original_hand.hole_cards);
                    assert_eq!(read_hand.actions.len(), original_hand.actions.len());
                    for (read_action, original_action) in
                        read_hand.actions.iter().zip(original_hand.actions.iter())
                    {
                        assert_eq!(
                            read_action.player_and_action,
                            original_action.player_and_action
                        );
                        assert_eq!(read_action.bet_amount, original_action.bet_amount);
                    }
                    assert_eq!(read_hand.timestamp, original_hand.timestamp);
                    assert_eq!(read_hand.board, original_hand.board);
                }
                (Event::StackUpdate(read_update), Event::StackUpdate(original_update)) => {
                    assert_eq!(read_update.seat, original_update.seat);
                    assert_eq!(read_update.stack, original_update.stack);
                }
                (Event::SeatUpdate(read_update), Event::SeatUpdate(original_update)) => {
                    assert_eq!(read_update.seat, original_update.seat);

                    // Compare the contents of player option
                    match (&read_update.player, &original_update.player) {
                        (Some(read_player), Some(original_player)) => {
                            assert_eq!(read_player.id, original_player.id);
                            assert_eq!(read_player.stack, original_player.stack);
                        }
                        (None, None) => {}
                        _ => panic!("Player option mismatch"),
                    }
                }
                _ => panic!("Unexpected event type"),
            }
        }
    }

    fn pack_player_and_action(player_index: u8, action_type: u8) -> u8 {
        (player_index << 4) | action_type
    }

    fn pack_board(cards: &[u8]) -> u32 {
        let mut board = 0u32;
        for (i, &card) in cards.iter().enumerate() {
            board |= (card as u32) << (i * 6);
        }
        board
    }
}
