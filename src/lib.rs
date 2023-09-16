// Author: Eskil Nyberg
// Based on IndaPlus22/task-3/chess_template by Viola Söderlund, modified by Isak Larsson

use std::fmt;

/// Enum for the current state of the game.
///
/// ### States
/// - `InProgress` describes that the game is initialized and playable. The game starts in this state.
/// This is the general state of the game unless the game is in check.
/// - `Check` describes that the game is currently in a check state that needs to be corrected.
/// In this state, `get_possible_moves()` returns a limited list of moves.
/// - `WaitingOnPromotionChoice` describes that the game is waiting for the user to choose which piece
/// the recently moved pawn should be promoted to.
/// - `GameOver` describes a finished game. All state-altering functions will not work in this state.
/// This state is reached by the game ending in some way. See `GameOverReason` for information about how.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    InProgress,
    Check,
    WaitingOnPromotionChoice,
    GameOver,
}

/// Enum for the reason the game game over:ed.
///
/// ### States
/// - `Checkmate` is reached when one player is checked and cannot by any means escape the check. `.get_active_colour()` is the losing colour and the opposite colour is the winning colour.
/// - `Stalemate` is reached when one player is not checked and has no possible legal moves.
/// - `_75MoveRule` is reached automatically when no move that captures a piece or moves a pawn has been made in 75 moves.
/// - `FivefoldRepetitionRule` is reached automatically when the same exact position has been reached five times.
/// - `DeadPosition` is reached automatically when what remains on the board is a well-known case of insufficient material.
/// - `MutualDraw` is reached if submitted manually by the `submit_draw` method.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameOverReason {
    Checkmate,
    Stalemate,
    _75MoveRule,
    FivefoldRepetitionRule,
    DeadPosition, // BUG: is only automatically entered if the board is in a well-known case of insufficient material
    MutualDraw, // may be reached if a player decides to use the 50 move rule or the threefold repetition rule
}

/// Enum for the colours of the board. Is implemented as an auxiliary state for by e.g. Piece and Game.
///
/// Contains the variants `White` and `Black`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    /// A function that returns the opposite colour
    fn opposite(colour: Colour) -> Colour {
        if colour == Colour::White {
            return Colour::Black;
        } else {
            return Colour::White;
        }
    }
}

/// Enum for the type of piece referenced. Is implemented by e.g. `Piece`.
///
/// Contains the variants `King`, `Queen`, `Rook`, `Knight`, `Bishop`, `Pawn`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Struct for some Piece.
///
/// Is used in the engine as an `Option<Piece>`-structure implementing None where there are no pieces and Some(Piece) where there are pieces.
///
/// Contains the fields piece_type of type PieceType and colour of type Colour.
pub struct Piece {
    pub piece_type: PieceType,
    pub colour: Colour,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Struct for some position. Contains the fields `row` and `col` corresponding to the row and col represented, individually,
/// as well as the field `idx` corresponding to the index of the position in the board array.
///
/// Note the implementations of `Position::new(row, col)`, `Position::new_from_idx(idx)` and `Position::parse_str(str)` perform error handling.
/// Every instance of Position should represent a legal position, or else the functions will return an Err Result.
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub idx: usize,
}

impl Position {
    /// Init-function that parses some position on the chessboard from the corresponding row and col as indices 0-7.
    ///
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board.
    pub fn new(row: usize, col: usize) -> Result<Position, String> {
        if row >= 8 || col >= 8 {
            let error = format!(
                "Invalid row: {} or col: {} input. Input should be between 0-7.",
                row, col
            );
            return Err(error);
        }

        return Ok(Position {
            row,
            col,
            idx: row * 8 + col,
        });
    }

    /// Init-function that parses some position on the chessboard from the corresponding array index 0-63.
    ///
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board.
    pub fn new_from_idx(idx: usize) -> Result<Position, String> {
        if idx > 63 {
            let error = format!("Invalid idx: {} input. Input should be between 0-63.", idx);
            return Err(error);
        }

        return Ok(Position {
            row: idx / 8,
            col: idx % 8,
            idx,
        });
    }

    /// Init-function that parses some position on the chessboard from a two character String on the format `XF` where `X` is a character a-h and `F` is a number 0-7. Performs trimming and caps-handling.
    ///
    /// Returns an `Ok(Position)`,
    /// or an `Err(&str)` describing the error if the input does not represent some part of the chess board.
    pub fn parse_str(str: &str) -> Result<Position, String> {
        let str_lowercase = str.to_lowercase(); // Performed to permit uppercase inputs. Saved in a memory to permit safe borrowing.
        let chars: Vec<char> = str_lowercase
            .trim() // Removes potential whitespaces passed to the function
            .chars()
            .collect(); // Creates the vector

        if chars.len() != 2 {
            return Err(String::from(format!("Input {} is of invalid length.", str)));
        }

        // parses the first character: the column; throws an error if the character is not a character between a-h
        let col: usize = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                let error = format!(
                    "First character '{}' of string invalid, should be some character between a-h",
                    chars[0]
                );
                return Err(error);
            }
        };

        // parses the second character: the row; throws an error if the character is not a number between 1-8
        // the function's return statement is nested within these if-statements
        let row: usize = match chars[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {
                let error = format!(
                    "Second character '{}' of string invalid, should be some number between 1-8",
                    chars[1]
                );
                return Err(error);
            }
        };

        return Position::new(row, col);
    }

    /// Function that modifies self by offset, given as a tuple (row-offset, col-offset)
    pub fn offset_self(&mut self, offset: (i32, i32)) -> Result<bool, String> {
        let row_result: i32 = self.row as i32 + offset.0;
        let col_result: i32 = self.col as i32 + offset.1;

        if row_result < 0 || row_result > 7 || col_result < 0 || col_result > 7 {
            return Err(String::from("New position not on board."));
        }

        // We are fine and complete the addition
        self.row = row_result as usize;
        self.col = col_result as usize;
        self.idx = self.row * 8 + self.col;
        return Ok(true);
    }
}

/// The game! This struct contains our accessible fields and methods.
///
/// % NOTE! Viewing in rustdoc, full descriptions for methods can be viewed under <a href="#implementations">Implementations</a> below. There you can also find links to the source code!
///
/// * `new()` which instantiates the game.
/// * `make_move(from_str, to_str)` which, if legal, makes a move from some position XF to some position XF and returns the resulting error or new GameState.
/// * `make_move_pos(from_pos, to_pos)` which, if legal, makes a move from some Position from_pos to some Position to_pos and returns the resulting error or new GameState.     (Recommended!)
/// * `get_game_state()` returns the state of the game.
/// * `get_game_over_reason()` returns the reason for the game over, or None if the game is in progress.
/// * `get_active_colour()` returns the active colour.
/// * `get_board()` returns the board as an array of `Option<Piece>`-s.
/// * `get_possible_moves(position, recursion_order)` returns a list of all possible moves for the piece at position.
/// * `set_promotion(piece)` should be called if the game is in GameState::WaitingOnPromotionChoice to indicate what piece to promote the last moved pawn to.
///
/// We also have some methods for situations where one might want to implement draws.
///
/// * `submit_draw()` lets you set the game as game-over-ed with GameOverReason::MutualDraw.
/// * `can_enact_threefold_repetition_rule()` checks if the threefold repetition rule is applicable.
/// * `can_enact_50_move_rule()` checks if the 50 move rule is applicable.
#[derive(Clone, Debug)] // The clone derivation is necessary as it is used by try_move
pub struct Game {
    state: GameState,
    game_over_reason: Option<GameOverReason>,
    active_colour: Colour,
    board: [Option<Piece>; 8 * 8],
    previous_boards: Vec<([Option<Piece>; 8 * 8], Colour, bool, bool, bool, bool, bool)>, // used for implementing the fivefold repetition rule (parameters are board, active_colour, last_move_to, pawn_just_moved_twice and the castling bools)
    moves_since_last_capture_or_moved_pawn: u8, // used for implementing the 75-move rule
    last_moved_to: Position,
    en_passant_pos: Position, // Is set to the most recent position to which a pawn can move to en passant capture another pawn
    pawn_just_moved_twice: bool, // Is set to true just after a pawn just moved twice (enabling en passant)
    white_king_can_a1_castle: bool, // Is set to false if the white king moves or is checked or if the a1 rook moves or is captured
    white_king_can_h1_castle: bool, // /- -/ but for white king and h1 rook
    black_king_can_a8_castle: bool, // /- -/ bot for black king and a8 rook
    black_king_can_h8_castle: bool, // /- -/ bot for black king and h8 rook
}

/// Here we implement the main functions of our game.
impl Game {
    /// This is a constant used in the function `try_move` that specifies how far the engine should check for Check-states.
    /// The value 2 should do since after 2 recursions, we have checked each user making the next move. In this time, we should discover all relevant Check-states.
    const MAX_RECURSIONS: i32 = 2;

    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        // generate the pieces
        let w_king = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::King,
        });
        let w_queen = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Queen,
        });
        let w_rook = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Rook,
        });
        let w_knight = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Knight,
        });
        let w_bishop = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Bishop,
        });
        let w_pawn = Some(Piece {
            colour: Colour::White,
            piece_type: PieceType::Pawn,
        });

        let b_king = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::King,
        });
        let b_queen = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Queen,
        });
        let b_rook = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Rook,
        });
        let b_knight = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Knight,
        });
        let b_bishop = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Bishop,
        });
        let b_pawn = Some(Piece {
            colour: Colour::Black,
            piece_type: PieceType::Pawn,
        });

        // initializing board array
        let board_init = [
            w_rook, w_knight, w_bishop, w_queen, w_king, w_bishop, w_knight, w_rook, w_pawn,
            w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, w_pawn, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, b_pawn,
            b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_pawn, b_rook, b_knight, b_bishop,
            b_queen, b_king, b_bishop, b_knight, b_rook,
        ];

        Game {
            /* initialise board, set active colour to white and state to in progress */
            state: GameState::InProgress,
            game_over_reason: None,
            active_colour: Colour::White,
            board: board_init,
            previous_boards: vec![(board_init, Colour::White, true, true, true, true, true); 0],
            moves_since_last_capture_or_moved_pawn: 0,
            last_moved_to: Position::new(0, 0).unwrap(), // arbitrary position, is updated before it is used
            en_passant_pos: Position::new(0, 0).unwrap(), // /- -/
            pawn_just_moved_twice: false,
            white_king_can_a1_castle: true,
            white_king_can_h1_castle: true,
            black_king_can_a8_castle: true,
            black_king_can_h8_castle: true,
        }
    }

    /// Use this method to end the game if the players mutually choose to submit a draw to the game.
    pub fn submit_draw(&mut self) {
        self.state = GameState::GameOver;
        self.game_over_reason = Some(GameOverReason::MutualDraw);
    }

    /// This method returns true if the threefold repetition rule can be enacted, otherwise false.
    pub fn can_enact_threefold_repetition_rule(&self) -> bool {
        return self
            .previous_boards
            .iter()
            .filter(|&n| {
                *n == (
                    self.board,
                    self.active_colour,
                    self.pawn_just_moved_twice,
                    self.white_king_can_a1_castle,
                    self.white_king_can_h1_castle,
                    self.black_king_can_a8_castle,
                    self.black_king_can_h8_castle,
                )
            })
            .count()
            >= 2;
    }

    /// This method returns true if the 50-move rule can be enacted, otherwise false.
    pub fn can_enact_50_move_rule(&self) -> bool {
        return self.moves_since_last_capture_or_moved_pawn >= 50;
    }

    /// If the current game state is InProgress or Check and the move is legal,
    /// move a piece and return the resulting state of the game. Performs trimmming and caps-handling.
    ///
    /// Updates all fields.
    pub fn make_move(&mut self, from_str: &str, to_str: &str) -> Result<GameState, String> {
        // parse from_str
        let from_pos = match Position::parse_str(&from_str) {
            Ok(result) => result,
            Err(string) => return Err(string),
        };

        // parse to_str
        let to_pos = match Position::parse_str(&to_str) {
            Ok(result) => result,
            Err(string) => return Err(string),
        };

        return self.make_move_pos(from_pos, to_pos);
    }

    /// (Variant of `make_move` that takes Positions as input instead.)
    /// If the current game state is InProgress or Check and the move is legal,
    /// move a piece and return the resulting state of the game.
    ///
    /// Updates all fields.
    pub fn make_move_pos(
        &mut self,
        from_pos: Position,
        to_pos: Position,
    ) -> Result<GameState, String> {
        // Checks that the game state is InProgress or Check, else throws an error.
        if !(self.state == GameState::InProgress || self.state == GameState::Check) {
            let error = format!("The game is not in a state where a move can be made. Currently, the state is {:?}.", self.state);
            return Err(error);
        }

        // check that the the piece is not None and is of the right colour
        match self.board[from_pos.idx] {
            None => {
                return Err(String::from(
                    "There is no piece on the square you are trying to move from.",
                ))
            }
            Some(piece) => {
                if piece.colour != self.active_colour {
                    return Err(String::from("It is not this colour's turn!"));
                }
            }
        }

        // Generates a list of all the legal moves that the piece in question can be performed.
        let possible_moves = self._get_possible_moves(from_pos, 0);

        if !possible_moves
            .iter() // Creates an iterable of positions.
            .any(|pos| pos == &to_pos)
        // Checks if our position is equal to some position in the list of possible moves. We use .any() since the objects may be different instances.
        {
            return Err(String::from("Illegal move. (This might mean that this piece cannot move this way, or that it puts your king in check!)"));
        } else {
            // Save pre-move board in previous_boards vector
            self.previous_boards.push((
                self.board,
                self.active_colour,
                self.pawn_just_moved_twice,
                self.white_king_can_a1_castle,
                self.white_king_can_h1_castle,
                self.black_king_can_a8_castle,
                self.black_king_can_h8_castle,
            ));
            // And we move the piece!
            let captured_piece: Option<Piece> = self.board[to_pos.idx]; // is None if none were captured
            self.board[to_pos.idx] = self.board[from_pos.idx];
            self.board[from_pos.idx] = None;
            // and check for any special case moves (castling, pawn moves (en passant stuff), king moves or rook moves)
            self.move_special_case_handler(from_pos, to_pos, captured_piece);
            // and save this movement for future reference
            self.last_moved_to = to_pos;
            // and update the active colour (NEEDS TO BE DONE BEFORE update_game_state()!)
            self.active_colour = Colour::opposite(self.active_colour);
            // and update the game state (to some variant of GameState)
            self.update_game_state();

            return Ok(self.state);
        }
    }

    /// Auxiliary function for make_move that handles special cases in the event of a move.
    /// Updates the fields `en_passant_pos`, `pawn_just_moved_twice`, `moves_since_last_capture_or_moved_pawn`, `white_king_can_a1_castle` etc.
    ///
    /// Note that updating the castling fields when the king is checked is handled by `update_game_state()`.
    /// This function is called after the move has been performed.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn move_special_case_handler(
        &mut self,
        from_pos: Position,
        to_pos: Position,
        captured_piece: Option<Piece>,
    ) {
        self.moves_since_last_capture_or_moved_pawn += 1; // is reset to 0 if applicable
        if captured_piece.is_some() {
            self.moves_since_last_capture_or_moved_pawn = 0;
        }
        match self.board[to_pos.idx].unwrap().piece_type {
            // We know to_pos is not empty
            PieceType::Pawn => {
                self.moves_since_last_capture_or_moved_pawn = 0;

                // For the pawn we need to check if the move was an en passant move
                // (in which case we should capture the correct pawn, which is not at to_pos)
                // or if the move triggers a state in which the opponent can en passant
                let dir: i32 = match self.board[to_pos.idx].unwrap().colour {
                    // the direction of pawn movement we need to compensate for
                    Colour::White => 1,
                    Colour::Black => -1,
                };
                // Check for en passant
                if self.pawn_just_moved_twice && to_pos == self.en_passant_pos {
                    self.pawn_just_moved_twice = false;
                    // in which case we make sure to capture the pawn which was en-passant:ed
                    let mut captured_pawn_pos: Position = to_pos.clone(); // the captured position is one square earlier so we need to offset this position
                    let _ = captured_pawn_pos.offset_self((-dir, 0)); // The en passant variables gaurantee this is safe.
                    self.board[captured_pawn_pos.idx] = None; // Capture the pawn
                }
                // else check if the move triggers an en passant state
                else if to_pos.row.abs_diff(from_pos.row) == 2 {
                    // (Occurs if a pawn moved two spaces forward.)
                    self.pawn_just_moved_twice = true;
                    self.en_passant_pos = to_pos.clone(); // the capturable position is one square earlier so we need to offset this position
                    let _ = self.en_passant_pos.offset_self((-dir, 0)); // Is safe due to how pawns are coded.
                } else {
                    self.pawn_just_moved_twice = false;
                }
            }
            PieceType::King => {
                self.pawn_just_moved_twice = false;
                // If the king performs a castling move, we need to move the rook as well.
                // If the king moves, we need to disable future castling for the colour that moved.

                // Check for castling and if necessary move rook.
                match to_pos.idx {
                    2 => {
                        if self.white_king_can_a1_castle {
                            // King is moving from e1, move the rook
                            self.board[3] = self.board[0];
                            self.board[0] = None;
                        }
                    }
                    6 => {
                        if self.white_king_can_h1_castle {
                            // King is moving from e1, move the rook
                            self.board[5] = self.board[7];
                            self.board[7] = None;
                        }
                    }
                    58 => {
                        if self.black_king_can_a8_castle {
                            // King is moving from e8, move the rook
                            self.board[59] = self.board[56];
                            self.board[56] = None;
                        }
                    }
                    62 => {
                        if self.black_king_can_a8_castle {
                            // King is moving from e8, move the rook
                            self.board[61] = self.board[63];
                            self.board[63] = None;
                        }
                    }
                    _default => {}
                }

                // Disable castling if the king moves.
                match self.active_colour {
                    Colour::White => {
                        self.white_king_can_a1_castle = false;
                        self.white_king_can_h1_castle = false;
                    }
                    Colour::Black => {
                        self.black_king_can_a8_castle = false;
                        self.black_king_can_h8_castle = false;
                    }
                }
            }
            PieceType::Rook => {
                self.pawn_just_moved_twice = false;
                // If the rook moves, we need to disable castling for the correct colour and rook.
                match from_pos.idx {
                    // indexes 0 = a1, 7 = h1, 56 = a8 and 63 = h8
                    0 => {
                        self.white_king_can_a1_castle = false;
                    }
                    7 => {
                        self.white_king_can_h1_castle = false;
                    }
                    56 => {
                        self.black_king_can_a8_castle = false;
                    }
                    63 => {
                        self.black_king_can_h8_castle = false;
                    }
                    _default => {} // do nothing
                }
            }
            _default => {
                self.pawn_just_moved_twice = false;
                // We also need to check if we capture either of the rooks at a1/h1/a8/h8,
                // in which case we can no longer castle with them.
                if captured_piece.is_some() && captured_piece.unwrap().piece_type == PieceType::Rook
                {
                    match to_pos.idx {
                        // indexes 0 = a1, 7 = h1, 56 = a8 and 63 = h8
                        0 => {
                            self.white_king_can_a1_castle = false;
                        }
                        7 => {
                            self.white_king_can_h1_castle = false;
                        }
                        56 => {
                            self.black_king_can_a8_castle = false;
                        }
                        63 => {
                            self.black_king_can_h8_castle = false;
                        }
                        _default => {} // do nothing
                    }
                }
            }
        }
    }

    /// Checks the current game state for the player of the active_colour and updates it. Expects the active colour to be updated to the next player's colour.
    ///
    /// Updates only the field `state` and the fields `white_king_can_a1_castle` etc.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn update_game_state(&mut self) {
        /*
        If there is a pawn that needs to be promoted (is at the end of the board),
        the method will put the game into GameState::WaitingOnPromotionChoice and skip the rest of the state-checking.
        This is safe because the promotion method set_promotion will call this method again at the end to set the state to one of the below values.
        */
        if self.state != GameState::GameOver {
            // Check if the user needs to promote a pawn by checking the piece at `last_moved_to`
            let last_moved_piece = self.board[self.last_moved_to.idx].unwrap(); // unwrap is safe due since last_moved_to is well-defined.
            if last_moved_piece.piece_type == PieceType::Pawn {
                // We only care for pawns of the active colour.
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                if last_moved_piece.colour == Colour::White && self.last_moved_to.row == 7 {
                    self.state = GameState::WaitingOnPromotionChoice;
                    return;
                } else if last_moved_piece.colour == Colour::Black && self.last_moved_to.row == 0 {
                    self.state = GameState::WaitingOnPromotionChoice;
                    return;
                }
            }
        }
        /* If the next thing to happen is not a promotion:
        If the current game state has occurred 4 times before, enact the fivefold repetition rule with GameState::GameOver.
        If the current game state is a known dead position, declare the game a draw with GameState::GameOver.
        If the king is in check and no correcting move can be made, the game is in checkmate with GameState::GameOver.
        If the king is in check and a correcting move can be made, the game is in check with GameState::Check.
        If the king is not in check yet no move can be made, the game is in stalemate with GameState::GameOver.
        If there have been 75 moves since the last captured piece or moved pawn, enact the 75-move rule with GameState::GameOver.
        If the king is not in check and some move can be made, the game is simply in progress with GameState::InProgress.

        Note that the method `can_make_legal_move` primarily uses the function `get_possible_moves` which checks whether
        some move puts the king in check when it is performed. A "possible" or "legal" move is thus defined as a move that
        can be performed without putting the king at risk.
        */

        // Fivefold repetition rule.
        if self
            .previous_boards
            .iter()
            .filter(|&n| {
                *n == (
                    self.board,
                    self.active_colour,
                    self.pawn_just_moved_twice,
                    self.white_king_can_a1_castle,
                    self.white_king_can_h1_castle,
                    self.black_king_can_a8_castle,
                    self.black_king_can_h8_castle,
                )
            })
            .count()
            >= 4
        {
            self.state = GameState::GameOver;
            self.game_over_reason = Some(GameOverReason::FivefoldRepetitionRule);
            return;
        }

        // Known dead positions (due to insufficient material)
        let remaining_pieces = self.board.iter().filter(|&n| n.is_some());
        let remaining_pieces_count = remaining_pieces.clone().count();
        if remaining_pieces_count < 5 {
            // Unwrap is safe now since we only have pieces left
            let mut bishop_count = 0;
            let mut knight_count = 0;
            for piece in remaining_pieces {
                match piece.unwrap().piece_type {
                    PieceType::Bishop => {
                        bishop_count += 1;
                    }
                    PieceType::Knight => {
                        knight_count += 1;
                    }
                    _default => {}
                }
            }
            if remaining_pieces_count == 3 && (bishop_count == 1 || knight_count == 1) {
                self.state = GameState::GameOver;
                self.game_over_reason = Some(GameOverReason::DeadPosition);
                return;
            } else if remaining_pieces_count == 4 && bishop_count == 2 {
                let mut bishop_loc = 64;
                for i in 0..63 {
                    if self.board[i].is_some()
                        && self.board[i].unwrap().piece_type == PieceType::Bishop
                    {
                        if bishop_loc == 64 {
                            bishop_loc = i;
                        } else if bishop_loc % 2 == i % 2 {
                            self.state = GameState::GameOver;
                            self.game_over_reason = Some(GameOverReason::DeadPosition);
                            return;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        if self.is_in_check(self.active_colour, 0) {
            if self.can_make_legal_move(self.active_colour) {
                if self.moves_since_last_capture_or_moved_pawn >= 75 {
                    self.state = GameState::GameOver;
                    self.game_over_reason = Some(GameOverReason::_75MoveRule);
                } else {
                    self.state = GameState::Check;
                }
                // If active_colour is in check, also disable castling for active_colour.
                match self.active_colour {
                    Colour::White => {
                        self.white_king_can_a1_castle = false;
                        self.white_king_can_h1_castle = false;
                    }
                    Colour::Black => {
                        self.black_king_can_a8_castle = false;
                        self.black_king_can_h8_castle = false;
                    }
                }
            } else {
                self.state = GameState::GameOver;
                self.game_over_reason = Some(GameOverReason::Checkmate);
            }
        } else {
            if self.can_make_legal_move(self.active_colour) {
                if self.moves_since_last_capture_or_moved_pawn >= 75 {
                    self.state = GameState::GameOver;
                    self.game_over_reason = Some(GameOverReason::_75MoveRule);
                } else {
                    self.state = GameState::InProgress;
                }
            } else {
                self.state = GameState::GameOver;
                self.game_over_reason = Some(GameOverReason::Stalemate);
            }
        }
    }

    /// Checks whether the king of colour `colour` is in check and returns a boolean. `recursion_order` should be set to 0 unless you know what you're doing.
    /// This is done by iterating over every piece of the opposite colour and checking whether it can move to the king.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS. If you are wondering whether the game is in state Check, please use `get_game_state` instead.
    /// This function is public such that it can be called by `get_possible_moves` on cloned instances.
    ///
    /// Note that this function calls `get_possible_moves` again which calls this function.
    /// To avoid infinite recursion, we pass the variable `recursion_order` which is incremented by `get_possible_moves`.
    fn is_in_check(&self, colour: Colour, recursion_order: i32) -> bool {
        let king_pos = self.find_king_pos(colour);

        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour != colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves =
                    self._get_possible_moves(Position::new_from_idx(i).unwrap(), recursion_order);
                if possible_moves
                    .iter() // Creates an iterable of positions.
                    .any(|pos| pos == &king_pos)
                // Checks if our position is equal to the list of possible moves. We use .any() since the objects may be different instances.
                {
                    return true;
                }
            } else {
                // Do nothing
            }
        }

        // If we have found no cases where the king is in check, the king is not in check.
        return false;
    }

    /// Checks whether the colour of parameter `colour` has some legal move it can make and returns a boolean.
    ///
    /// This primarily relies on the function `get_possible_moves` which implements checking whether some move would put the king in check.
    /// Is implemented in checkmate-checking.
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn can_make_legal_move(&self, colour: Colour) -> bool {
        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().colour == colour {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                let possible_moves =
                    self._get_possible_moves(Position::new_from_idx(i).unwrap(), 0);
                if possible_moves.len() > 0 {
                    // We have found at least one possible move and return true
                    return true;
                }
            }
        }

        // We have, after iterating over every piece, found no possible move and return false
        return false;
    }

    /// Finds the king of colour `colour`'s position and returns it as a Position
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn find_king_pos(&self, colour: Colour) -> Position {
        for (i, piece) in self.board.iter().enumerate() {
            if piece.is_none() {
                // Do nothing
            } else if piece.unwrap().piece_type == PieceType::King
                && piece.unwrap().colour == colour
            {
                // Unwrapping piece is safe here since it is not none.
                // Unwrapping Position::new_from_idx(i) is safe here since the board is well defined.
                return Position::new_from_idx(i).unwrap();
            }
        }
        panic!("The king is not on the board! Something is wrong.");
    }

    /// Set the piece type that a peasant becames following a promotion. Performs trimming and caps-handling.
    /// Accepted inputs are `"queen", "rook", "bishop" or "knight"`.
    pub fn set_promotion(&mut self, piece: String) -> Result<GameState, String> {
        if self.state != GameState::WaitingOnPromotionChoice {
            return Err(String::from(format!(
                "The game is not currently waiting on a promotion. Currently, the state is {:?}.",
                self.state
            )));
        }
        let piece_lowercase = piece.to_lowercase();

        let piece_type = match piece_lowercase.trim() {
            "queen" => PieceType::Queen,
            "rook" => PieceType::Rook,
            "bishop" => PieceType::Bishop,
            "knight" => PieceType::Knight,
            "king" => return Err(String::from("You can't promote a pawn to a king!")),
            "pawn" => return Err(String::from("You can't promote a pawn to a pawn!")),
            _ => {
                return Err(String::from(format!(
                    "Invalid input '{}'.",
                    piece_lowercase
                )))
            }
        };

        self.board[self.last_moved_to.idx] = Some(Piece {
            piece_type,
            colour: self.board[self.last_moved_to.idx].unwrap().colour,
        });

        // update active colour
        if self.active_colour == Colour::Black {
            self.active_colour = Colour::White;
        } else {
            self.active_colour = Colour::Black;
        }

        self.update_game_state();
        return Ok(self.state);
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// Get the game over reason. Is None if the game is not over.
    pub fn get_game_over_reason(&self) -> Option<GameOverReason> {
        self.game_over_reason
    }

    /// Get the current game state.
    pub fn get_active_colour(&self) -> Colour {
        self.active_colour
    }

    /// Get a reference to the board as a slice of length 8 * 8 of `Option<Piece>`-s
    pub fn get_board(&self) -> &[Option<Piece>; 8 * 8] {
        return &self.board;
    }

    /// If a piece is standing on the given tile, this method returns all possible new positions of that piece as a vector of positions.
    ///
    /// Takes the argument `pos` of type Position.
    pub fn get_possible_moves(&self, pos: Position) -> Vec<Position> {
        // This method just relays the position to _get_possible_moves with recursion_order 0.
        return self._get_possible_moves(pos, 0);
    }

    /// If a piece is standing on the given tile, this method returns all possible new positions of that piece.
    ///
    /// Takes the arguments `pos` of type Position and `recursion_order`. Put `recursion_order` to 0 if you do not know what you are doing.
    /// `recursion_order` is an auxiliary variable that prevents the function from checking for potential Check-states further in the future than MAX_RECURSIONS.
    fn _get_possible_moves(&self, pos: Position, mut recursion_order: i32) -> Vec<Position> {
        // Increment recursion_order. See docstring for details.
        recursion_order += 1;

        // Get piece. If it is None, it cannot move so return an empty vector.
        let piece: Piece = match self.board[pos.idx] {
            None => return vec![],
            Some(piece) => piece,
        };

        // Start listing possible moves.
        let mut possible_moves: Vec<Position> = Vec::with_capacity(60);

        // For each piece_type, follow some set of rules.
        /* Design philosophy:
            For every direction that a piece should move in, generate an offset or a set of offsets for that direction.
            Then, iterate over every direction using the function try_move (see the function for details) which returns two booleans:
                legal_move - if the move is legal; then it is added to the possible_moves vector
                engine_should_continue - bool describing if the move should cause this method to halt further movement in the same direction
            So, we iterate over each offset in every direction until we reach a point where engine_should_continue is false, and then we change direction.

            Note that the pawn implementation is hacked! Pawns do not work the same way, but their behavior abides to the checks performed by the above booleans.
            See their specific implementation for details.

            Note that trial.0 refers to legal_move and trial.1 refers to engine_should_continue.
        */
        match piece.piece_type {
            PieceType::King => {
                // Kings can move all directions but only one distance.
                // Kings can also castle if nothing has happened in the game that disables this.
                // (See commens on `struct Game` fields for details.)
                // See the comment above the match-case for details on the implementation.

                // Normal movement.
                for offset in [
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, 1),
                    (0, -1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ] {
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                }

                // Castling.
                // (One case per castling opportunity, since they have hardcoded positioning.)
                match piece.colour {
                    Colour::White => {
                        let king_pos = Position::new(0, 4).unwrap();
                        if self.white_king_can_a1_castle {
                            // Boolean is true iff the king is at e1 and the rook is at a1.
                            // Check if b1 [idx 1], c1 [idx 2], and d1 [idx 3] are free.
                            if self.board[1].is_none()
                                && self.board[2].is_none()
                                && self.board[3].is_none()
                            {
                                // In that case check if the king is checked on the way to castling at c1.
                                let mut ok = true;
                                for i in 1..=2 {
                                    let offset = (0, -i);
                                    let trial = self.try_move(king_pos, offset, recursion_order);
                                    if !trial.0 {
                                        ok = false;
                                    }
                                }

                                // If it is not checked on the way, enable castling at c1.
                                if ok {
                                    possible_moves.push(Position::new(0, 2).unwrap());
                                }
                            }
                        }
                        if self.white_king_can_h1_castle {
                            // Boolean is true iff the king is at e1 and the rook is at h1.
                            // Check if f1 [idx 5] and g1 [idx 6] are free.
                            if self.board[5].is_none() && self.board[6].is_none() {
                                // In that case check if the king is checked on the way to castling at g1.
                                let mut ok = true;
                                for i in 1..=2 {
                                    let offset = (0, i);
                                    let trial = self.try_move(king_pos, offset, recursion_order);
                                    if !trial.0 {
                                        ok = false;
                                    }
                                }

                                // If it is not checked on the way, enable castling at g1.
                                if ok {
                                    possible_moves.push(Position::new(0, 6).unwrap());
                                }
                            }
                        }
                    }
                    Colour::Black => {
                        let king_pos = Position::new(7, 4).unwrap();
                        if self.black_king_can_a8_castle {
                            // Boolean is true iff the king is at e8 and the rook is at a8.
                            // Check if b8 [idx 57], c8 [idx 58] and d8 [idx 59] are free.
                            if self.board[57].is_none()
                                && self.board[58].is_none()
                                && self.board[59].is_none()
                            {
                                // In that case check if the king is checked on the way to castling at c8.
                                let mut ok = true;
                                for i in 1..=2 {
                                    let offset = (0, -i);
                                    let trial = self.try_move(king_pos, offset, recursion_order);
                                    if !trial.0 {
                                        ok = false;
                                    }
                                }

                                // If it is not checked on the way, enable castling at c8.
                                if ok {
                                    possible_moves.push(Position::new(7, 2).unwrap());
                                }
                            }
                        }
                        if self.black_king_can_h8_castle {
                            // Boolean is true iff the king is at d8 and the rook is at h8.
                            // Check if f8 [idx 61] and g8 [idx 62] are free.
                            if self.board[61].is_none() && self.board[62].is_none() {
                                // In that case check if the king is checked on the way to castling at g8.
                                let mut ok = true;
                                for i in 1..=2 {
                                    let offset = (0, i);
                                    let trial = self.try_move(king_pos, offset, recursion_order);
                                    if !trial.0 {
                                        ok = false;
                                    }
                                }

                                // If it is not checked on the way, enable castling at g8.
                                if ok {
                                    possible_moves.push(Position::new(7, 6).unwrap());
                                }
                            }
                        }
                    }
                }
            }
            PieceType::Queen => {
                // Queens can move all directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, 1),
                    (0, -1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Bishop => {
                // Bishops can move all diagonal directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Knight => {
                // Knight can move according to eight movesets.
                // See the comment above the match-case for details on the implementation.
                for offset in [
                    (2, 1),
                    (2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                    (-2, 1),
                    (-2, -1),
                ] {
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                }
            }
            PieceType::Rook => {
                // Rooks can move all non-diagonal directions and however far they like. (The board is size 8.)
                // See the comment above the match-case for details on the implementation.
                for dir in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
                    for len in 1..8 {
                        let offset = (dir.0 * len, dir.1 * len);
                        let trial = self.try_move(pos, offset, recursion_order);
                        if trial.0 {
                            let mut ok_pos = pos.clone();
                            ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                            possible_moves.push(ok_pos);
                        }

                        if !trial.1 {
                            break;
                        }
                    }
                }
            }
            PieceType::Pawn => {
                /* This pawn-implementation is hacky :)
                    We find the direction (positive or negative) and then iterate
                    i) forward in that direction
                    ii) to the sides

                    In the forward direction we allow all moves which don't return a false boolean engine_should_continue from try_move (trial.1 in the code).
                    This is because that indicates that we either i) have run into the end of the board or ii) have run into a piece.
                    The first option isn't relevant for pawns, and the second the method try_move thinks is legal but actually isn't, since pawns can't capture forward.

                    For double-step-checking, we break the loop after the first iteration here if there is a piece on the way or if the piece is not on the second row.


                    In the diagonal direction we do the opposite! We ONLY allow moves for which try_move returns a false boolean engine_should_continue,
                    with the same methodology. If engine_should_continue is false, we would be capturing a piece.

                    See the docstring above the match-case for context.
                */

                let dir: i32;
                let mut on_first_row = false;
                if piece.colour == Colour::White {
                    dir = 1;
                    if pos.row == 1 {
                        on_first_row = true;
                    }
                } else {
                    dir = -1;
                    if pos.row == 6 {
                        on_first_row = true;
                    }
                }

                // forward direction
                for (i, j) in [(1, 0), (2, 0)] {
                    let offset: (i32, i32) = (i * dir, j);
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 && trial.1 {
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                    if !on_first_row || !trial.1 {
                        // break if it is not on the first row or if there was a piece in the way
                        break;
                    }
                }

                // diagonal direction
                for (i, j) in [(1, 1), (1, -1)] {
                    let offset: (i32, i32) = (i * dir, j);
                    let trial = self.try_move(pos, offset, recursion_order);
                    if trial.0 && !trial.1 {
                        // en passant is included in this if-check, see try_move for details
                        let mut ok_pos = pos.clone();
                        ok_pos.offset_self(offset).unwrap(); // unwrap is safe after try_move
                        possible_moves.push(ok_pos);
                    }
                }
            }
        }
        return possible_moves;
    }

    /// This function tries to move a piece from old_pos to the offset (i32, i32). Does not check whether pieces are in the way for this move, but it does
    /// check whether it puts the own king in check.
    /// Takes as input `recursion_order` too, which is an integer describing which order in the recursion this iteration of try_move is.
    /// If the iteration is higher than MAX_RECURSIONS, this function will not check whether a move implies putting the king in check.
    ///
    /// Returns two booleans, one bool indicating whether the move was legal (internally legal_move)
    /// and another bool indicating whether the engine should continue checking for legal moves in the same direction (internally engine_should_continue)
    ///
    /// SHOULD ONLY BE CALLED BY INTERNAL FUNCTIONS.
    fn try_move(
        &self,
        old_pos: Position,
        offset: (i32, i32),
        recursion_order: i32,
    ) -> (bool, bool) {
        if self.board[old_pos.idx].is_none() {
            panic!(
                "try_move was called trying to move a piece from a tile where there is no piece!"
            );
        }

        /* The philosophy for this function is that we generate a clone of the own game, perform the move in that game and see where that takes us.
            We also perform error-handling for the offset (if it is off the board) and check whether there is a piece in the way.
            If there is a piece in the way, we check that it is of the opposite color (a.k.a. capture-able)
            and in that case return that the engine should not continue.

            If a move is found to be almost legal, a.k.a. moves to an empty piece or a piece of the opposite color, this function will check whether
            the move puts the own king in check by calling is_check on the new board. This step is skipped if the recursion order is greater than
            MAX_RECURSIONS.

            There are comments guiding you through the if-clauses below if you need to read the code.
        */

        // Unwrapping is safe since it is not none.
        let player_colour = self.board[old_pos.idx].unwrap().colour;

        // Generate new position and check if it is in the board
        let mut new_pos = old_pos.clone();
        match new_pos.offset_self(offset) {
            Err(_) => return (false, false), // If the new position is outside of the board, it is not valid and the engine should change direction.
            _ => (),                         // continue
        };

        // Clone into a new game to try the movement in that game
        let mut game_after_movement = self.clone();
        game_after_movement.board[new_pos.idx] = game_after_movement.board[old_pos.idx];
        game_after_movement.board[old_pos.idx] = None;
        game_after_movement.active_colour = Colour::opposite(game_after_movement.active_colour);

        // Check piece movement on the new board
        let legal_move: bool;
        let engine_should_continue: bool;
        match self.board[new_pos.idx] {
            // If there is no piece in the new slot, check if it is possibly an en passent capturable slot,
            // else return false if the king is in check after movement or else true. Return true that the engine should keep checking the same direction.
            None => {
                // En passant is applicable if the opponent just moved their pawn twice, and the piece that's being moved is a pawn.
                // Which position can be captured en passant is defined by the Game.en_passant_pos which is set by the make_move variants.
                // The game is in en_passant_bool == true the move right after the double-move of a pawn, otherwise false.
                // "If we are trying to move a pawn, the game is in en-passant-mode and the square we are moving to is the en_passant_pos..."
                if self.board[old_pos.idx].is_some()
                    && self.board[old_pos.idx].unwrap().piece_type == PieceType::Pawn
                    && self.pawn_just_moved_twice
                    && new_pos == self.en_passant_pos
                {
                    engine_should_continue = false; // get_possible_moves will interpret this as a valid move
                } else {
                    engine_should_continue = true;
                }
                if recursion_order < Game::MAX_RECURSIONS {
                    legal_move = !game_after_movement.is_in_check(player_colour, recursion_order);
                } else {
                    legal_move = true;
                }
                ();
            }
            // If there is a piece in the new slot, the engine should not keep checking the same direction...
            Some(piece) => {
                engine_should_continue = false;
                // ... and the move is not legal if the piece is of the player's colour
                if piece.colour == player_colour {
                    legal_move = false;
                }
                // ... else the move is legal if the king is not in check after movement
                else {
                    if recursion_order < Game::MAX_RECURSIONS {
                        legal_move =
                            !game_after_movement.is_in_check(player_colour, recursion_order);
                    } else {
                        legal_move = true;
                    }
                }
            }
        }

        return (legal_move, engine_should_continue);
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:------------------------------:|
/// | wR  wKn wB  wK  wQ  wB  wKn wR |
/// | wP  wP  wP  wP  wP  wP  wP  wP |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | *   *   *   *   *   *   *   *  |
/// | bP  bP  bP  bP  bP  bP  bP  bP |
/// | bR  bKn bB  bK  bQ  bB  bKn bR |
/// |:------------------------------:|
///
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // init output, the string we'll be coding our format to
        let mut output = String::new();

        // start with the top row
        output.push_str("|:------------------------------:|\n");

        // for every Option<piece> in board, print a representation. Also, for every beginning of a row i % 8 == 0 and end of a row i & 8 == 7 add corresponding slices.
        for (i, piece) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                output.push_str("|");
            }

            if piece.is_none() {
                output.push_str(" *  "); // there is no piece here, add an asterisk
            } else {
                // from here, unwrapping is safe since the piece is not None
                // add initial spacing
                output.push_str(" ");

                // match dict for Colour representation
                output.push_str(match piece.unwrap().colour {
                    Colour::White => "w",
                    Colour::Black => "b",
                });

                // match dict for PieceType representation
                output.push_str(match piece.unwrap().piece_type {
                    PieceType::King => "K ",
                    PieceType::Queen => "Q ",
                    PieceType::Bishop => "B ",
                    PieceType::Knight => "Kn",
                    PieceType::Rook => "R ",
                    PieceType::Pawn => "P ",
                });
            }

            if i % 8 == 7 {
                output.push_str("|\n");
            }
        }

        // end with the bottom row
        output.push_str("|:------------------------------:|");

        write!(f, "{}", output)
    }
}

impl fmt::Display for Colour {
    // Make the formatter print colours fancily outside of debug mode.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod lib_tests;
