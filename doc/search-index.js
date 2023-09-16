var searchIndex = JSON.parse('{\
"lib":{"doc":"","t":"NNNNENNDNEENNNNNDEDNNNNNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["Bishop","Black","Check","Checkmate","Colour","DeadPosition","FivefoldRepetitionRule","Game","GameOver","GameOverReason","GameState","InProgress","King","Knight","MutualDraw","Pawn","Piece","PieceType","Position","Queen","Rook","Stalemate","WaitingOnPromotionChoice","White","_75MoveRule","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","can_enact_50_move_rule","can_enact_threefold_repetition_rule","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","col","colour","eq","eq","eq","eq","eq","eq","equivalent","equivalent","equivalent","equivalent","equivalent","equivalent","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","get_active_colour","get_board","get_game_over_reason","get_game_state","get_possible_moves","hash","hash","hash","hash","hash","hash","idx","into","into","into","into","into","into","into","make_move","make_move_pos","new","new","new_from_idx","offset_self","parse_str","piece_type","row","set_promotion","submit_draw","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id"],"q":[[0,"lib"]],"d":["","","","","Enum for the colours of the board. Is implemented as an …","","","The game! This struct contains our accessible fields and …","","Enum for the reason the game game over:ed.","Enum for the current state of the game.","","","","","","Struct for some Piece.","Enum for the type of piece referenced. Is implemented by …","Struct for some position. Contains the fields <code>row</code> and <code>col</code> …","","","","","","","","","","","","","","","","","","","","","This method returns true if the 50-move rule can be …","This method returns true if the threefold repetition rule …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Get the current game state.","Get a reference to the board as a slice of length 8 * 8 of …","Get the game over reason. Is None if the game is not over.","Get the current game state.","If a piece is standing on the given tile, this method …","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","If the current game state is InProgress or Check and the …","(Variant of <code>make_move</code> that takes Positions as input …","Init-function that parses some position on the chessboard …","Initialises a new board with pieces.","Init-function that parses some position on the chessboard …","Function that modifies self by offset, given as a tuple …","Init-function that parses some position on the chessboard …","","","Set the piece type that a peasant becames following a …","Use this method to end the game if the players mutually …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[6,5,3,4,0,4,4,0,3,0,0,3,6,6,4,6,0,0,0,6,6,4,3,5,4,3,4,5,6,7,8,1,3,4,5,6,7,8,1,1,1,3,4,5,6,7,8,1,3,4,5,6,7,8,1,8,7,3,4,5,6,7,8,3,4,5,6,7,8,3,4,5,5,6,7,8,1,1,3,4,5,6,7,8,1,1,1,1,1,1,3,4,5,6,7,8,8,3,4,5,6,7,8,1,1,1,8,1,8,8,8,7,8,1,1,3,4,5,6,7,8,1,5,1,3,4,5,6,7,8,1,3,4,5,6,7,8,1,3,4,5,6,7,8,1],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,2],[1,2],[3,3],[4,4],[5,5],[6,6],[7,7],[8,8],[1,1],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,[[3,3],2],[[4,4],2],[[5,5],2],[[6,6],2],[[7,7],2],[[8,8],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[],2],[[3,9],10],[[4,9],10],[[5,9],10],[[5,9],10],[[6,9],10],[[7,9],10],[[8,9],10],[[1,9],10],[[1,9],10],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,5],[1,[[12,[[11,[7]]]]]],[1,[[11,[4]]]],[1,3],[[1,8],[[13,[8]]]],[[3,14]],[[4,14]],[[5,14]],[[6,14]],[[7,14]],[[8,14]],0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[1,15,15],[[17,[3,16]]]],[[1,8,8],[[17,[3,16]]]],[[18,18],[[17,[8,16]]]],[[],1],[18,[[17,[8,16]]]],[8,[[17,[2,16]]]],[15,[[17,[8,16]]]],0,0,[[1,16],[[17,[3,16]]]],[1],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],16],[[],16],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],17],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19],[[],19]],"c":[],"p":[[3,"Game"],[15,"bool"],[4,"GameState"],[4,"GameOverReason"],[4,"Colour"],[4,"PieceType"],[3,"Piece"],[3,"Position"],[3,"Formatter"],[6,"Result"],[4,"Option"],[15,"array"],[3,"Vec"],[8,"Hasher"],[15,"str"],[3,"String"],[4,"Result"],[15,"usize"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};