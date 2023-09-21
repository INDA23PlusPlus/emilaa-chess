/// ### Chess piece strucure.
#[derive(Copy, Clone)]
struct ChessPiece {
    piece_id: i8,
    color: i8,
    has_moved: bool,
    has_moved_two: bool
}

impl ChessPiece {
    pub fn new() -> Self {
        return ChessPiece { piece_id: 0, color: 0, has_moved: false, has_moved_two: false };
    }

    pub fn white(id: i8) -> ChessPiece {
        return ChessPiece { piece_id: id, color: -1, has_moved: false, has_moved_two: false };
    }

    pub fn black(id: i8) -> ChessPiece {
        return ChessPiece { piece_id: id, color: 1, has_moved: false, has_moved_two: false };
    }
}
/**
### Chess board structure.
## Fields:
`game_end`: boolean that shows if the game has ended or not.
*/
pub struct ChessBoard {
    board: [ChessPiece; 64],
    game_started: bool,
    pub game_end: bool,
    white_turn: bool,
    en_passant: bool,
    white_castling: (bool, bool),
    black_castling: (bool, bool),
    white_king_index: usize,
    black_king_index: usize
}

impl ChessBoard {
    /**
    ### Get a fresh chess board.
    ## Returns:
    New standard chess board with pieces.
    */
    pub fn new() -> Self {
        let mut b: ChessBoard = ChessBoard { 
            board: [ChessPiece::new(); 64],
            game_started: false,
            game_end: false,
            white_turn: true,
            en_passant: false,
            white_castling: (true, true),
            black_castling: (true, true),
            white_king_index: 56,
            black_king_index: 31
        };

        b.board[0] = ChessPiece::black(2);
        b.board[1] = ChessPiece::black(3);
        b.board[2] = ChessPiece::black(4);
        b.board[3] = ChessPiece::black(5);
        b.board[4] = ChessPiece::black(6);
        b.board[5] = ChessPiece::black(4);
        b.board[6] = ChessPiece::black(3);
        b.board[7] = ChessPiece::black(2);

        b.board[56] = ChessPiece::white(2);
        b.board[57] = ChessPiece::white(3);
        b.board[58] = ChessPiece::white(4);
        b.board[59] = ChessPiece::white(5);
        b.board[60] = ChessPiece::white(6);
        b.board[61] = ChessPiece::white(4);
        b.board[62] = ChessPiece::white(3);
        b.board[63] = ChessPiece::white(2);

        for i in 8..16 {
            b.board[i] = ChessPiece::black(1);
            b.board[i+40] = ChessPiece::white(1);
        }

        return b;
    }

    ///### Reset the board.
    pub fn reset(&mut self) {
        self.board = ChessBoard::new().board;
        self.game_started = false;
        self.game_end = false;
        self.white_turn = true;
        self.white_castling = (true, true);
        self.black_castling = (true, true);
        self.white_king_index = 60;
        self.black_king_index = 4;
    }

    /**
    ### Move a piece using algebraic notation.
    The function moves the requested piece if nothing went wrong. <br>
    This function converts the algebraic notation into indices and calls move by index.
    ## Parameters:
    `from`: two letter string, representing the piece you want to move. <br>
    `to`: two letter string, representing the destination tile.
    ## Returns:
    `false` if anything goes wrong, otherwise `true`
    ## Example:
    `move_by_algebraic("e8", "e7")` would attempt to move a piece from e8 to e7.
    */
    pub fn move_by_algebraic(&mut self, from: &str, to: &str) -> bool {
        if from.is_empty() || from.len() > 2 { 
            println!("Piece to move was not provided..."); 
            return false;
        }

        if to.is_empty() || to.len() > 2 { 
            println!("Destination was not provided...");
            return false;
        }

        let file_f = from.as_bytes()[0].to_ascii_lowercase() as i8;
        let rank_f = from.as_bytes()[1].to_ascii_lowercase() as i8;
        let file_t = to.as_bytes()[0].to_ascii_lowercase() as i8;
        let rank_t = to.as_bytes()[1].to_ascii_lowercase() as i8;

        if file_f < 97 || file_f > 104 { println!("Bad file: {}", from); return false; }
        if rank_f < 49 || rank_f > 56  { println!("Bad rank: {}", from); return false; }
        if file_t < 97 || file_t > 104 { println!("Bad file: {}", to);   return false; }
        if rank_t < 49 || rank_t > 56  { println!("Bad rank: {}", to);   return false; }

        let from_index: i8 = file_f - 97 + (rank_f - 56).abs() * 8;
        let to_index: i8 = file_t - 97 + (rank_t - 56).abs() * 8;

        return self.move_by_index(from_index as usize, to_index as usize);
    }

    /**
    ### Move a piece using the boards indices.
    The function moves the requested piece if nothing went wrong.
    ## Important:
    If a king is captured or a checkmate is detected, the `game_end` field is set to false. 
    ## Parameters:
    `from`: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    `to`: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if anything goes wrong, otherwise `true`
    ## Example:
    `move_by_index(4, 12)` would attempt to move a piece from index 4 to 12, or e8 to e7 in algebraic notation.
    */
    pub fn move_by_index(&mut self, from: usize, to: usize) -> bool {
        if from == to {
            println!("Can't move to same tile...");
            return false;
        }

        if !self.game_started && self.board[from].color == 1 {
            println!("White goes first...");
            return false;
        }

        if self.board[from].color == -1 && !self.white_turn {
            println!("Black is supposed to play...");
            return false;
        }
        
        if self.board[from].color == 1 && self.white_turn {
            println!("White is supposed to play...");
            return false;
        }

        if self.is_tile_empty(from) {
            println!("Attempting to move from an empty tile...");
            return false;
        }

        if from == 4 && to == 0 { if self.attempt_castling(false, true) { self.update(); return true; } }
        if from == 4 && to == 7 { if self.attempt_castling(false, false) { self.update(); return true; } }
        if from == 60 && to == 56 { if self.attempt_castling(true, true) { self.update(); return true; } }
        if from == 60 && to == 63 { if self.attempt_castling(true, false) { self.update(); return true; } }

        if !self.is_move_legal(from, to) {
            println!("Illegal move...");
            return false;
        }
        
        if !self.is_tile_same_color(to, from) && self.en_passant {
            self.en_passant = false;
            self.board[(to as i8 - self.board[from].color * 8) as usize] = ChessPiece::new();
        }

        if !self.is_tile_same_color(to, from) { self.board[to as usize] = ChessPiece::new(); }

        self.board.swap(from as usize, to as usize);

        self.update();

        if self.simulate_checkmate(true, self.white_king_index) ||
           self.simulate_checkmate(false, self.black_king_index)
        {
            println!("A king was checkmated...");
            self.game_end = true;
        }

        return true;
    }

    /**
    ### Determines if requested move is legal.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if no legal move was found, otherwise `true`
    */
    fn is_move_legal(&mut self, from: usize, to: usize) -> bool {
        match self.board[from].piece_id {
            1 => { return self.check_pawn_move(from, to); }
            2 => { return self.check_rook_move(from, to); }
            3 => { return self.check_knight_move(from, to); }
            4 => { return self.check_bishop_move(from, to); }
            
            // Queen essentially moves like the bishop and rook, combined.
            5 => { return self.check_bishop_move(from, to) || self.check_rook_move(from, to); }
            6 => { return self.check_king_move(from, to); }

            _ => { return false; }
        }
    }

    /**
    ### Determines if the pawn move is legal.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_pawn_move(&mut self, from: usize, to: usize) -> bool {
        if !self.board[from].has_moved                              && 
           to as i8 == from as i8 + self.board[from].color * 16i8   &&
           self.is_tile_empty(to) 
        {
            self.board[from].has_moved = true;
            self.board[from].has_moved_two = true;
            return true;
        }
        
        if self.board[from].color == 1 {
            if to > from + 9 || to < from + 7 { return false; }
        } else {
            if to > from - 7 || to < from - 9 { return false; }
        }
        
        if !self.check_en_passant(from, to)      &&
           ( self.is_tile_empty(to)              || 
           self.is_tile_same_color(to, from) )   &&
           to as i8 != from as i8 + self.board[from].color * 8
        {
            return false;
        }
        
        if self.board[from].has_moved_two { self.board[from].has_moved_two = false; }
        self.board[from].has_moved = true;

        self.promote_pawn(from, to);

        return true;
    }

    /**
    ### Determines if the rook move is legal.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_rook_move(&mut self, from: usize, to: usize) -> bool {
        let file: i8    = self.index_file(from as i8);
        let rank: i8    = self.index_rank(from as i8);
        let t_file: i8  = self.index_file(to as i8);
        let t_rank: i8  = self.index_rank(to as i8);

        if ( file != t_file && rank != t_rank ) || 
           self.is_tile_same_color(to, from) 
        {
            return false;
        }

        let dir: i8 = if file == t_file { 
            (t_rank - rank).signum() * 8 
        } else { 
            (t_file - file).signum() 
        };

        let mut i: i8 = from as i8 + dir;

        while i != to as i8 {
            if !self.is_tile_empty(i as usize) {
                return false;
            }
            i += dir;
        }

        if self.is_tile_same_color(to, from) { return false; }

        return true;
    }

    /**
    ### Determines if the knight move is legal.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_knight_move(&mut self, from: usize, to: usize) -> bool {
        let file: i8 = self.index_file(from as i8);
        let rank: i8 = self.index_rank(from as i8);
        let mut indices: [i8; 8] = [
            from as i8 - 17i8,
            from as i8 - 15i8,
            from as i8 - 10i8,
            from as i8 - 6i8,

            from as i8 + 17i8,
            from as i8 + 15i8,
            from as i8 + 10i8,
            from as i8 + 6i8
        ];

        if file < 2 { indices[2] = i8::MAX; indices[7] = i8::MAX; }
        if file < 1 { indices[0] = i8::MAX; indices[5] = i8::MAX; }
        if file > 5 { indices[3] = i8::MAX; indices[6] = i8::MAX; }
        if file > 6 { indices[1] = i8::MAX; indices[4] = i8::MAX; }

        if rank < 2 { indices[0] = i8::MAX; indices[1] = i8::MAX; }
        if rank < 1 { indices[2] = i8::MAX; indices[3] = i8::MAX; }
        if rank > 5 { indices[4] = i8::MAX; indices[5] = i8::MAX; }
        if rank > 6 { indices[6] = i8::MAX; indices[7] = i8::MAX; }

        if !indices.contains(&(to as i8)) || self.is_tile_same_color(to, from) { return false; }

        return true;
    }

    /**
    ### Determines if the bishop move is legal.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_bishop_move(&mut self, from: usize, to: usize) -> bool {
        let dir: i8 = (self.index_rank(to as i8) - self.index_rank(from as i8)).signum() * 8;
        let step: i8 = (self.index_file(to as i8) - self.index_file(from as i8)).signum();

        if dir == 0 || step == 0 { return false; }

        let mut i = from as i8 + dir + step;
        let mut last_file = self.index_file(i);

        while i >= 0 && i < 64 && i != to as i8 {
            if (last_file - self.index_file(i)).abs() == 7 { return false; }
            if !self.is_tile_empty(i as usize) { return false; }

            last_file = self.index_file(i);
            i += dir + step;
        }

        if self.is_tile_same_color(from, to) { return false; }

        return true;
    }

    /**
    ### Determines if the king move is legal.
    ## Note:
    If the move was legal, all castling rights are revoked to corresponding team.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_king_move(&mut self, from: usize, to: usize) -> bool {
        let file: i8 = self.index_file(from as i8);
        let rank: i8 = self.index_rank(from as i8);

        let mut i: i8 = -9;
        while i <= 9 {
            if i == -6 { i = -1; }
            if i == 2 { i = 7; }

            if (i + from as i8) > 63 || (i + from as i8) < 0 { i += 1; continue; }
            if (self.index_file(i + from as i8) - file).abs() > 1 ||
               (self.index_rank(i + from as i8) - rank).abs() > 1
            {
                i += 1;
                continue;
            }

            if i + from as i8 == to as i8 && !self.is_tile_same_color(to, from) {
                let white_king: bool = self.board[from].color == -1;
                if self.simulate_check(white_king, to) { return false; }

                if white_king {
                    self.white_king_index = to;
                    self.white_castling = (false, false);
                } else {
                    self.black_king_index = to;
                    self.black_castling = (false, false);
                }

                return true; 
            }

            i += 1;
        }

        return false;
    }

    /**
    ### Determines if a king would be checked at an index.
    ## Parameters:
    `white`: Calculate for white (`true`) king or black (`false`) king.
    `index`: The index to test for.
    ## Returns:
    `true` if the king would be in check, otherwise `false`.
    */
    fn simulate_check(&mut self, white: bool, index: usize) -> bool {
        let test_color: i8 = if white { 1 } else { -1 };

        for i in 0..64 {
            if self.board[i].piece_id == 0 || self.board[i].color != test_color { continue; }
            if self.is_move_legal(i, index) {
                return true;
            }
        }

        return false;
    }

    /**
    ### Determines if a king would be checkmated at an index.
    ## Parameters:
    `white`: Calculate for white (`true`) king or black (`false`) king.
    `index`: The index to test for.
    ## Returns:
    `true` if the king would be in checkmate, otherwise `false`.
    */
    fn simulate_checkmate(&mut self, white: bool, index: usize) -> bool {
        let mut ways_out: std::vec::Vec<i8> = vec![];
        let file: i8 = self.index_file(index as i8);
        let rank: i8 = self.index_rank(index as i8);

        let mut i: i8 = -9;
        while i <= 9 {
            if i == -6 { i = -1; }
            if i == 2 { i = 7; }

            if (i + index as i8) > 63 || (i + index as i8) < 0 || i == 0 { i += 1; continue; }
            if (self.index_file(i + index as i8) - file).abs() > 1 ||
            (self.index_rank(i + index as i8) - rank).abs() > 1
            {
                i += 1;
                continue;
            }

            if !self.simulate_check(white, (i+index as i8) as usize) { ways_out.push(i + index as i8) }
            
            i += 1;
        }

        return ways_out.is_empty();
    }

    /**
    ### Determines if the pawn can perform an "<i>en passant</i>" capture.
    ## Parameters:
    from: number between 0 (inclusive) and 64, representing the piece you want to move. <br>
    to: number between 0 (inclusive) and 64, representing the destination tile.
    ## Returns:
    `false` if the move was illegal, otherwise `true`
    */
    fn check_en_passant(&mut self, from: usize, to: usize) -> bool {
        let piece: &ChessPiece = &self.board[from];
        if to as i8 - piece.color * 8 < 0 || to as i8 - piece.color * 8 > 63 { return false; }
        let target: &ChessPiece = &self.board[(to as i8 - piece.color * 8) as usize];

        if target.piece_id != 1 || piece.color == target.color { return false; }
        if !target.has_moved_two { return false; }

        self.en_passant = true;
        return true;
    }

    /**
    ### Determines if a pawn shall be promoted.
    If the pawn has reached to other side of the board, it is promoted to a queen.
    ## Note:
    Only promotes to a queen.
    */
    fn promote_pawn(&mut self, from: usize, to: usize) {
        if self.board[from].color == 1 && to > 55 && to < 64 {
            self.board[from].piece_id = 5;
        }

        if self.board[from].color == -1 && to < 8 {
            self.board[from].piece_id = 5;
        }
    }

    /**
    ### Update castling rights.
    If a rook moved, castling rights are revoked from either the king side or queen side.
    ## Note:
    Doesn't update for the king, since the `check_king_move()` function already does that.
    */
    fn update_castling_rights(&mut self) {
        if self.board[56].piece_id != 2 && self.board[60].piece_id == 6 { self.white_castling.0 = false; }
        if self.board[63].piece_id != 2 && self.board[60].piece_id == 6 { self.white_castling.1 = false; }
        if self.board[0].piece_id != 2 && self.board[4].piece_id == 6 { self.black_castling.0 = false; }
        if self.board[7].piece_id != 2 && self.board[4].piece_id == 6 { self.black_castling.1 = false; }
    }

    /**
    ### Attempts to perform castling.
    ## Returns:
    `false` if anything went wrong (no rights, something was in the way), otherwise `true`
    */
    fn attempt_castling(&mut self, white: bool, queen_side: bool) -> bool {

        if white && queen_side && !self.white_castling.0 { return false; }
        if white && !queen_side && !self.white_castling.1 { return false; }
        if !white && queen_side && !self.black_castling.0 { return false; }
        if !white && !queen_side && !self.black_castling.1 { return false; }

        let king_pos: i8 = if white { 60 } else { 4 };
        let steps: i8 = if queen_side { 4 } else { 3 };
        let dir: i8 = if queen_side { -1 } else { 1 };

        for i in 1..steps { 
            if !self.is_tile_empty((king_pos + i * dir) as usize) {
                return false;
            }
        }

        if white && queen_side { self.board.swap(60, 58); self.board.swap(56, 59); self.white_castling = (false, false); }
        if white && !queen_side { self.board.swap(60, 62); self.board.swap(63, 61); self.white_castling = (false, false); }
        if !white && queen_side { self.board.swap(4, 2); self.board.swap(0, 3); self.white_castling = (false, false); }
        if !white && !queen_side { self.board.swap(4, 6); self.board.swap(7, 5); self.white_castling = (false, false); }

        return true;
    }

    /**
    ### Update the game state.
    ## Note:
    Calls `update_castling_rights()` as well.
    */
    fn update(&mut self) {
        self.white_turn = !self.white_turn;
        if !self.game_started { self.game_started = true; }
        self.update_castling_rights();
    }

    ///### Check if a tile is empty.
    fn is_tile_empty(&self, index: usize) -> bool {
        return self.board[index].piece_id == 0;
    }
    
    ///### Check if the tile has a piece with the same color as requested.
    fn is_tile_same_color(&self, index: usize, piece_index: usize) -> bool {
        return self.board[index].color == self.board[piece_index].color;
    }

    ///### Convert index to file.
    fn index_file(&self, index: i8) -> i8 { return index % 8; }

    ///### Convert index to rank.
    fn index_rank(&self, index: i8) -> i8 { return (index - (index % 8)) / 8 }

    ///### Print the chess board.
    pub fn print(&mut self) {
        let colors: [&str; 2] = [ "30;47;1", "47;40;1" ];

        print!("\x1b[38;5;130;1m+----------------+\n|\x1b[39;49;0m");
        for i in 0..64 {
            let index: usize = self.board[i].color.clamp(0, 1) as usize;

            if self.board[i].piece_id != 0 {
                    print!("\x1b[{}m{} \x1b[39;49;0m", 
                           colors[index], 
                           self.id_to_char(self.board[i].piece_id));
            } else {
                print!("  ");
            }

            if (i + 1) % 8 == 0 { print!("\x1b[38;5;130m|\n|\x1b[39;49;0m"); }
        }
        print!("\x1b[38;5;130m\r+----------------+\x1b[39;49;0m\n\n");
    }

    ///### Convert piece id to corresponding character.
    fn id_to_char(&self, id: i8) -> char {
        match id {
            1 => { return 'P' }
            2 => { return 'R' }
            3 => { return 'k' }
            4 => { return 'B' }
            5 => { return 'Q' }
            6 => { return 'K' }

            _ => { return ' '; }
        }
    }  
}