# Chess - Emil Aalto

Usage:<br/>
Create a board with `let mut board = chess::ChessBoard::new()`.<br/>

You can move pieces with `move_by_algebraic()` or `move_by_index()`.<br/>

If any of them return false, an illegal move was made.<br/>

You can use `reset()` to reset the board and `print()` to print the board.<br/>