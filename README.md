# eskilny-task-03-chess

This library is a Rust chess library that supports move generation/validation, board modifications, and check/checkmate/stalemate detection. Virtually every rule outlined in [Rules of Chess - Wikipedia](https://en.wikipedia.org/wiki/Rules_of_chess) is implemented.

```markdown
Rust version: 1.72.0
Library version: 1.0.0
This library uses semantic versioning.
License: BSD-3
```

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
chess_engine = { git = "https://github.com/IndaPlus23/eskilny-task-03-chess/eskilny-task-03-chess.git" }
```

If your IDE doesn't do it for you, run `cargo build` to fetch the library.

## Welcome

Welcome to my chess library!

This library has a fully fledged rustdoc available [at this page](https://indaplus23.github.io/eskilny-task-03-chess/doc/lib/index.html) where you can view docstrings, source code and everything else without feeling so overwhelmed. Try it out!

## API

The recommended use of this library is that you interact with the structs Game and Position. For an example of how to interact with the library, see [main.rs](src/main.rs) which is the interface I used for playing the game while debugging it. The tests in [lib_tests.rs](src/lib_tests.rs) may also provide some insight. Examples are also available in docstrings.

`Game` is the chess engine! See [Game in the rustdoc for details.](https://indaplus23.github.io/eskilny-task-03-chess/doc/lib/struct.Game.html).

Position is an auxiliary struct that provides nice parsing methods for working with the rank (row) and file (column) of some position interchangably with the corresponding index.
It has nice constructor methods:

- `Position::new(row,col)` from the row or column on the format 0-7
- `Position::new_from_idx(idx)` from the index on the format 0-63 (great if you're iterating over the board)
- `Position::parse_str(str)` from a string on the format XF where X is a character a-h and F is a number 1-8

Details are available in the rustdoc!

This library stores the chess board as an array of `Option<Piece>`-s. In each `Piece` you'll find its `PieceType` and `Colour`.
If you want to represent the state of the board in some way, learn to work with this array of pieces! Working via Position is recommended.

You can get the current board with the method `Game::get_board()`. Moves can be made with the methods `make_move(...)` or `make_move_pos(...)`, the latter of which I recommend.

Once again, check out [the rustdoc](https://indaplus23.github.io/eskilny-task-03-chess/doc/lib/index.html)!

Good luck!

Report any bugs as issues plz <3

```markdown
Known bugs:
- Threefold and fivefold repetition rules comparisons where the relevant private fields of Game are different but the available moves are not do not count toward the multiplicity.
```
