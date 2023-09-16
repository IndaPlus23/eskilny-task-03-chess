# Chess engine, DD1337 week 3-4

## Eskil Nyberg, eskilny

```markdown
Rust version: 1.72.0
Library version: 1.0.0
This library uses semantic versioning.
License: BSD-3
```

Welcome to my chess library!

This library has a fully fledged rustdoc available [at this page](https://htmlpreview.github.io/https://github.com/IndaPlus23/eskilny-task-03-chess/blob/main/doc/lib/index.html) where you can view docstrings, source code and everything else without feeling so overwhelmed. Try it out!

The recommended use of this library is that you interact with the structs Game and Position. For an example of how to interact with the library, see [main.rs](src/main.rs) which is the interface I used for playing the game while debugging it. The tests in [lib_tests.rs](src/lib_tests.rs) may also provide some insight.

Game is the game library! See [Game in the rustdoc for details.](https://htmlpreview.github.io/https://github.com/IndaPlus23/eskilny-task-03-chess/blob/main/doc/lib/struct.Game.html)
This library is implemented fully. It doesn't currently offer any way to view past game states or display recent moves,
but other than that the library implements virtually every rule present on [Rules of Chess - Wikipedia](https://en.wikipedia.org/wiki/Rules_of_chess), including en passant, castling and virtually every rule that draws the game.

Position is an auxiliary struct that provides nice parsing methods for working with the row and column of some position interchangably with the corresponding index.
It has nice initialization methods:

- `Position::new(row,col)` from the row or column on the format 0-7
- `Position::new_from_idx(idx)` from the index on the format 0-63 (great if you're iterating over the board)
- `Position::parse_str(str)` from a string on the format XF where X is a character a-h and F is a number 1-8

This library stores the chess board as an array of Option\<Piece\>-s. In each Piece you'll find its PieceType and Colour.
If you want to represent the state of the board in some way, learn to work with this array of pieces! As I said, working via Position is recommended.

You can get the current board via the function `Game::get_board()`. Moves can be made with the methods `make_move(...)` or `make_move_pos(...)`, the latter of which I recommend.

Once again, check out [the rustdoc](https://htmlpreview.github.io/https://github.com/IndaPlus23/eskilny-task-03-chess/blob/main/doc/lib/index.html)!

Good luck!

Report any bugs as issues plz <3

```markdown
Known bugs:
- Threefold and fivefold repetition rules comparisons where the relevant private fields of Game are different but the available moves are not do not count toward the multiplicity.
- The draw-scenario dead position is not triggered for cases which are dead positions but not due to insufficient materials. See Wikipedia for details.
```
