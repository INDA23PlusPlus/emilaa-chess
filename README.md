# Chess - Emil Aalto

### Usage:<br/>
Create a board with `let mut board = chess::ChessBoard::new()`.<br/>

You can move pieces with `move_by_algebraic()` or `move_by_index()`.<br/>

If any of them return false, an illegal move was made.<br/>

You can use `reset()` to reset the board and `print()` to print the board.<br/>

You can get a copy of the board with `get_board()` which returns an array of tuples with a size of 64. The tuples contain what piece and what color is on the tile. See codes bellow.

### Codes:<br/>

| Code | Player | Color |
|:----:|--------|-------|
|-1    | -      | White |
|0     | None   | None  |
|1     | Pawn   | Black |
|2     | Rook   | -     |
|3     | Knight | -     |
|4     | Bishop | -     |
|5     | Queen  | -     |
|6     | King   | -     |