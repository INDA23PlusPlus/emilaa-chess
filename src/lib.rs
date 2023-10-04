use std::collections::HashMap;

/// Chess piece structure.
#[derive(Copy, Clone)]
struct Piece {
    id: i8,
    team: i8,
    moved: bool,        // Pawns only.
    moved_twice: bool   // Pawns only.
}

impl Piece {
    /// Return new piece.
    fn new(id: i8, color: i8) -> Piece {
        if color < -1 || color > 1 { panic!("Bad color..."); }

        return Piece { id: id, team: color, moved: false, moved_twice: false };
    }

    /// Get a white piece.
    fn white(id: i8) -> Piece {
        if id < 1 || id > 6 { panic!("Bad piece..."); }
        return Self::new(id, -1);
    }

    /// Get a black piece.
    fn black(id: i8) -> Piece {
        if id < 1 || id > 6 { panic!("Bad piece..."); }
        return Self::new(id, 1);
    }

    /// Get an empty / dummy piece.
    fn empty() -> Piece {
        return Self::new(0, 0);
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Flags {
    None = 0,
    TwoSteps,
    EnPassant,
    Capture,
    Kastling,
    Qastling
}

/// Chess board structure.
pub struct ChessBoard {
    board: [[Piece; 8]; 8],
    game_ended: bool,
    white_turn: bool,
    /// White castling, king side.
    wkcr: bool,
    /// White castling, queen side.
    wqcr: bool,
    /// Black castling, king side.
    bkcr: bool,
    /// Black castling, queen side.
    bqcr: bool,
    promoting: bool,
    promoting_index: (usize, usize),
    move_list: HashMap<(usize, usize), Vec<(usize, usize, Flags)>>
}

impl ChessBoard {
    /// Get a new board.
    pub fn new() -> ChessBoard {
        let mut board = ChessBoard {
            board: [[Piece::empty(); 8]; 8],
            game_ended: false,
            white_turn: true,
            wkcr: true,
            wqcr: true,
            bkcr: true,
            bqcr: true,
            promoting: false,
            promoting_index: (usize::MAX, usize::MAX),
            move_list: HashMap::new()
        };

        board.board[0][0] = Piece::black(2);
        board.board[0][1] = Piece::black(3);
        board.board[0][2] = Piece::black(4);
        board.board[0][3] = Piece::black(5);
        board.board[0][4] = Piece::black(6);
        board.board[0][5] = Piece::black(4);
        board.board[0][6] = Piece::black(3);
        board.board[0][7] = Piece::black(2);

        board.board[7][0] = Piece::white(2);
        board.board[7][1] = Piece::white(3);
        board.board[7][2] = Piece::white(4);
        board.board[7][3] = Piece::white(5);
        board.board[7][4] = Piece::white(6);
        board.board[7][5] = Piece::white(4);
        board.board[7][6] = Piece::white(3);
        board.board[7][7] = Piece::white(2);

        for i in 0..8usize {
            board.board[1][i] = Piece::black(1);
            board.board[6][i] = Piece::white(1);
        }

        board.gen_moves();

        return board;
    }

    /// Reset the board.
    pub fn reset(&mut self) {
        self.board = ChessBoard::new().board;
        self.game_ended = false;
        self.white_turn = true;
        self.wkcr = true;
        self.wqcr = true;
        self.bkcr = true;
        self.bqcr = true;
        self.promoting = false;
        self.promoting_index = (usize::MAX, usize::MAX);
        self.move_list = HashMap::new();
    }

    /** 
    Check if "it's joever."                             <br/>
    Returns:                                            <br/>
    `true` if the game has ended, otherwise `false`
    */
    pub fn is_game_ended(&self) -> bool { return self.game_ended; }

    /**
    Check if a pawn can be promoted.                            <br/>
    Returns:                                                    <br/>
    `true` if the a pawn can be promoted, otherwise `false`
    */
    pub fn can_promote(&self) -> bool { return self.promoting; }

    /**
    Get the team that is playing.                   <br/>
    Returns:                                        <br/>
    `true` if white is playing, otherwise `false`
    */
    pub fn get_player(&self) -> bool { return self.white_turn; }

    /**
    Try to promote a pawn.                              <br/>
    Returns:                                            <br/>
    `true` if a pawn got promoted, otherwise `false`.
    */
    pub fn promote(&mut self, id: i8) -> bool {
        if self.promoting && id < 6 && id > 1 {
            self.board[self.promoting_index.1][self.promoting_index.0].id = id;
            self.promoting = false;
            self.promoting_index = (usize::MAX, usize::MAX);
            self.white_turn = !self.white_turn;
            if self.gen_moves() { self.game_ended = true; }
            return true;
        }
        
        return  false;
    }

    /**
    Get a copy of the board.                                                            <br/>
    Returns:                                                                            <br/>
    A flat array of tuples with size 64. First element is the piece id, second is color.
    */
    pub fn get_board(&self) -> [(i8, i8); 64] {
        let mut b: [(i8, i8); 64] = [(0,0); 64];

        for y in 0..8usize {
            for x in 0..8usize {
                b[y*8+x] = (self.board[y][x].id, self.board[y][x].team);
            }            
        }

        return b;
    }

    /** Move piece by algebraic notation.                          <br/>
    Parameters:                                                    <br/>
    `from`: File from A to H and rank from 1 to 8. Example: "b1"   <br/>
    `to`: File from A to H and rank from 1 to 8. Example: "a3"     <br/>
    Returns:                                                       <br/>
    `true` on success, otherwise `false`
    */
    pub fn move_by_algebraic(&mut self, from: &str, to: &str) -> bool {
        if from.is_empty() || from.len() > 2 || to.is_empty() || to.len() > 2 { return false; }

        let file_f = from.as_bytes()[0].to_ascii_lowercase() as i8;
        let rank_f = from.as_bytes()[1].to_ascii_lowercase() as i8;
        let file_t = to.as_bytes()[0].to_ascii_lowercase() as i8;
        let rank_t = to.as_bytes()[1].to_ascii_lowercase() as i8;

        if file_f < 97 || file_f > 104 || rank_f < 49 || rank_f > 56 { return false; }
        if file_t < 97 || file_t > 104 || rank_t < 49 || rank_t > 56 { return false; }

        let from_: i8 = file_f - 97 + (rank_f - 56).abs() * 8;
        let to_: i8 = file_t - 97 + (rank_t - 56).abs() * 8;

        return self.move_by_index(from_ as usize, to_ as usize);
    }

    /** Move piece by index.                <br/>
    Parameters:                             <br/>
    `from`: Index to move from 0 ≤ i < 64   <br/>
    `to`: Index to move from 0 ≤ i < 64     <br/>
    Returns:                                <br/>
    `true` on success, otherwise `false`
    */
    pub fn move_by_index(&mut self, from: usize, to: usize) -> bool {
        if from > 63 || to > 63 || from == to { return false; }
        if self.promoting { return false; }
        let from_: (usize, usize) = ((from as i8 % 8) as usize, ((from as i8 - from as i8 % 8) / 8) as usize);
        let to_: (usize, usize) = ((to as i8 % 8) as usize, ((to as i8 - to as i8 % 8) / 8) as usize);

        if self.board[from_.1][from_.0].team == -1 && !self.white_turn { return false; }
        if self.board[from_.1][from_.0].team ==  1 &&  self.white_turn { return false; }

        let get = self.move_list.get(&from_);
        let moves: &Vec<(usize, usize, Flags)>;

        if get.is_some() {
            moves = get.unwrap();
        } else {
            return false;
        }

        let mut move_type: Flags = Flags::None;
        let mut found: bool = false;
        for m in moves.iter() {
            if m.0 == to_.0 && m.1 == to_.1 {
                found = true;
                move_type = m.2;
                break;
            }
        }

        if !found { return false; }

        if move_type == Flags::Capture { self.board[to_.1][to_.0] = Piece::empty(); }
        if move_type == Flags::TwoSteps { self.board[from_.1][from_.0].moved_twice = true; }
        if move_type == Flags::EnPassant {
            let team = self.board[from_.1][from_.0].team;
            let ep = (to_.0, (to_.1 as i8 - team) as usize);
            self.board[ep.1][ep.0] = Piece::empty();
        }

        if !self.board[from_.1][from_.0].moved { 
            self.board[from_.1][from_.0].moved = true;

            if self.board[from_.1][from_.0].id == 2 {
                if self.board[from_.1][from_.0].team == -1 {
                    if from_.0 == 0 { self.wqcr = false; }
                    if from_.0 == 7 { self.wkcr = false; }
                } else {
                    if from_.0 == 0 { self.bqcr = false; }
                    if from_.0 == 7 { self.bkcr = false; }
                }
            }

            if self.board[from_.1][from_.0].id == 6 && (move_type != Flags::Kastling && move_type != Flags::Qastling) {
                if self.board[from_.1][from_.0].team == -1 {
                    self.wqcr = false;
                    self.wkcr = false;
                } else {
                    self.bqcr = false;
                    self.bkcr = false;
                }
            }
        }
        
        if self.board[from_.1][from_.0].moved_twice && move_type != Flags::TwoSteps { self.board[from_.1][from_.0].moved_twice = false; }

        // Handle castling.
        if move_type == Flags::Kastling {
            if self.wkcr && self.board[from_.1][from_.0].team == -1 {
                let mut tmp = self.board[from_.1][from_.0];
                self.board[from_.1][from_.0] = self.board[to_.1][to_.0];
                self.board[to_.1][to_.0] = tmp;
                tmp = self.board[7][7];
                self.board[7][7] = self.board[7][5];
                self.board[7][5] = tmp;
                self.board[7][5].moved = true;

                self.wkcr = false;
                self.wqcr = false;
            }

            if self.bkcr && self.board[from_.1][from_.0].team == 1 {
                let mut tmp = self.board[from_.1][from_.0];
                self.board[from_.1][from_.0] = self.board[to_.1][to_.0];
                self.board[to_.1][to_.0] = tmp;
                tmp = self.board[0][7];
                self.board[0][7] = self.board[0][5];
                self.board[0][5] = tmp;
                self.board[0][5].moved = true;

                self.bkcr = false;
                self.bqcr = false;
            }
        } else if move_type == Flags::Qastling {
            if self.wqcr && self.board[from_.1][from_.0].team == -1 {
                let mut tmp = self.board[from_.1][from_.0];
                self.board[from_.1][from_.0] = self.board[to_.1][to_.0];
                self.board[to_.1][to_.0] = tmp;
                tmp = self.board[7][0];
                self.board[7][0] = self.board[7][3];
                self.board[7][3] = tmp;
                self.board[7][3].moved = true;

                self.wkcr = false;
                self.wqcr = false;
            }

            if self.bqcr && self.board[from_.1][from_.0].team == 1 {
                let mut tmp = self.board[from_.1][from_.0];
                self.board[from_.1][from_.0] = self.board[to_.1][to_.0];
                self.board[to_.1][to_.0] = tmp;
                tmp = self.board[0][0];
                self.board[0][0] = self.board[0][3];
                self.board[0][3] = tmp;
                self.board[0][3].moved = true;

                self.bkcr = false;
                self.bqcr = false;
            }
        } else {
            let tmp = self.board[from_.1][from_.0];
            self.board[from_.1][from_.0] = self.board[to_.1][to_.0];
            self.board[to_.1][to_.0] = tmp;
        }

        // Has a pawn reached the other side?
        if self.board[to_.1][to_.0].id == 1 && ((self.board[to_.1][to_.0].team == -1 && to_.1 == 0) || (self.board[to_.1][to_.0].team == 1 && to_.1 == 7))
        {
            self.promoting = true;
            self.promoting_index = to_;
            return true;
        }

        self.white_turn = !self.white_turn;
        if self.gen_moves() { self.game_ended = true; }
        
        return true;
    }
    /**
    Generate moves for current team.                                            <br/>
    Returns:                                                                    <br/>
    `true` if movelist is empty, equivalent to a checkmate, otherwise `false`
    */
    fn gen_moves(&mut self) -> bool {
        self.move_list.clear();

        let team: i8 = if self.white_turn { -1 } else { 1 };
        let mut team_indices: Vec<(usize, usize)> = vec![];

        for y in 0..8usize {
            for x in 0..8usize {
                if self.board[y][x].team == team { team_indices.push((x,y)); }
            }
        }

        // This should not happen.
        if team_indices.is_empty() { 
            self.game_ended = true; 
            panic!("No pieces in team. This should not happen...");    
        }

        for i in team_indices.iter() {
            let current_index: (i8, i8) = (i.0 as i8, i.1 as i8);
            let mut moves: Vec<(usize, usize, Flags)> = vec![];
            
            match self.board[i.1][i.0].id {
                1 => { moves.append(&mut self.gen_pawn_move(current_index, team)); }
                2 => { moves.append(&mut self.gen_rook_move(current_index, team)); }
                3 => { moves.append(&mut self.gen_knight_move(current_index, team)); }
                4 => { moves.append(&mut self.gen_bishop_move(current_index, team)); }
                5 => { moves.append(&mut self.gen_queen_move(current_index, team)); }
                6 => { moves.append(&mut self.gen_king_move(current_index, team)); }

                _ => { }
            }

            self.move_list.insert(i.to_owned(), moves);
        }

        self.validate_moves(team);

        return self.move_list.is_empty();
    }

    /// Validate generated moves.
    /// TODO:
    /// Fix to use indices.
    fn validate_moves(&mut self, team: i8) {
        let mut bad_moves: Vec<(usize, usize, usize)> = vec![];
        let mut king_indices: (usize, usize) = (usize::MAX, usize::MAX);

        for y in 0..8usize {
            for x in 0..8usize {
                if self.board[y][x].team == team && self.board[y][x].id == 6 { 
                    king_indices = (x, y);
                    break;
                }
            }
        }

        if king_indices == (usize::MAX, usize::MAX) {
            panic!("This shouldn't happen...");
        }

        for k in self.move_list.iter() {
            let v = k.1;

            for (index, m) in v.iter().enumerate() {
                let p0 = self.board[k.0.1][k.0.0];
                let p1 = self.board[m.1][m.0];
                let mut ki = king_indices;

                if p0.id == 6 { ki = (m.0, m.1); }
                
                // Swap
                if m.2 == Flags::Capture { self.board[m.1][m.0] = Piece::empty() }
                let tmp = self.board[m.1][m.0];
                self.board[m.1][m.0] = self.board[k.0.1][k.0.0];
                self.board[k.0.1][k.0.0] = tmp;

                // Enemy tries to kill the king.
                // Get moves on new board.
                let mut enemy_moves: HashMap<(usize, usize), Vec<(usize, usize, Flags)>> = HashMap::new();
                let mut enemy_indices: Vec<(usize, usize)> = vec![];

                for y in 0..8usize {
                    for x in 0..8usize {
                        if self.board[y][x].team == -team { enemy_indices.push((x,y)); }
                    }
                }

                for i in enemy_indices.iter() {
                    let current_index: (i8, i8) = (i.0 as i8, i.1 as i8);
                    let mut moves: Vec<(usize, usize, Flags)> = vec![];
                    
                    match self.board[i.1][i.0].id {
                        1 => { moves.append(&mut self.gen_pawn_move(current_index, -team)); }
                        2 => { moves.append(&mut self.gen_rook_move(current_index, -team)); }
                        3 => { moves.append(&mut self.gen_knight_move(current_index, -team)); }
                        4 => { moves.append(&mut self.gen_bishop_move(current_index, -team)); }
                        5 => { moves.append(&mut self.gen_queen_move(current_index, -team)); }
                        6 => { moves.append(&mut self.gen_king_move(current_index, -team)); }
        
                        _ => { }
                    }
        
                    enemy_moves.insert(i.to_owned(), moves);
                }

                for ek in enemy_moves.iter() {
                    let ev = ek.1;

                    for em in ev {
                        if em.0 == ki.0 && em.1 == ki.1 && !bad_moves.contains(&(k.0.0, k.0.1, index)) {
                            bad_moves.push((k.0.0, k.0.1, index));
                            break;
                        }
                    }
                }
                
                // Swap back
                self.board[k.0.1][k.0.0] = p0;
                self.board[m.1][m.0] = p1;
            }
        }

        // Delete all bad moves.
        for bm in bad_moves.iter() {
            self.move_list.get_mut(&(bm.0, bm.1)).unwrap()[bm.2] = (usize::MAX, usize::MAX, Flags::None);
        }

        for k in self.move_list.iter_mut() {
            k.1.retain(|&m| m.0 != usize::MAX && m.1 != usize::MAX);
        }

        self.move_list.retain(|&_, v| !v.is_empty());
    }

    /// Generate pawn moves.
    fn gen_pawn_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let kernel: [(i8, i8); 4] = [(0, 1 * team), (0, 2 * team), (-1, 1 * team), (1, 1 * team)];
        let mut moves: Vec<(usize, usize, Flags)> = vec![];

        // Forward move.
        if self.within_board((index.0 + kernel[0].0, index.1 + kernel[0].1)) &&
           self.empty_tile(((index.0 + kernel[0].0) as usize, (index.1 + kernel[0].1) as usize)) {

            moves.push(((index.0 + kernel[0].0) as usize, (index.1 + kernel[0].1) as usize, Flags::None));
        }

        // Double forward move.
        if self.within_board((index.0 + kernel[1].0, index.1 + kernel[1].1)) &&
           !self.board[index.1 as usize][index.0 as usize].moved &&
           self.empty_tile(((index.0 + kernel[1].0) as usize, (index.1 + kernel[1].1) as usize)) {

            moves.push(((index.0 + kernel[1].0) as usize, (index.1 + kernel[1].1) as usize, Flags::TwoSteps));
        }

        // Diagonals
        if self.within_board((index.0 + kernel[2].0, index.1 + kernel[2].1)) &&
           self.enemy_tile(((index.0 + kernel[2].0) as usize, (index.1 + kernel[2].1) as usize), team) {

            moves.push(((index.0 + kernel[2].0) as usize, (index.1 + kernel[2].1) as usize, Flags::Capture));
        }

        if self.within_board((index.0 + kernel[3].0, index.1 + kernel[3].1)) &&
           self.enemy_tile(((index.0 + kernel[3].0) as usize, (index.1 + kernel[3].1) as usize), team) {

            moves.push(((index.0 + kernel[3].0) as usize, (index.1 + kernel[3].1) as usize, Flags::Capture));
        }

        // En passant
        if self.within_board((index.0 + kernel[2].0, index.1 + kernel[2].1)) &&
           self.empty_tile(((index.0 + kernel[2].0) as usize, (index.1 + kernel[2].1) as usize)) &&
           self.enemy_tile(((index.0 + kernel[2].0) as usize, (index.1 + kernel[2].1 - team) as usize), team) &&
           self.board[(index.1 + kernel[2].1 - team) as usize][(index.0 + kernel[2].0) as usize].moved_twice {

            moves.push(((index.0 + kernel[2].0) as usize, (index.1 + kernel[2].1) as usize, Flags::EnPassant));
        }

        if self.within_board((index.0 + kernel[3].0, index.1 + kernel[3].1)) &&
           self.empty_tile(((index.0 + kernel[3].0) as usize, (index.1 + kernel[3].1) as usize)) &&
           self.enemy_tile(((index.0 + kernel[3].0) as usize, (index.1 + kernel[3].1 - team) as usize), team) &&
           self.board[(index.1 + kernel[3].1 - team) as usize][(index.0 + kernel[3].0) as usize].moved_twice {

            moves.push(((index.0 + kernel[3].0) as usize, (index.1 + kernel[3].1) as usize, Flags::EnPassant));
        }

        return moves;
    }

    // Generate rook moves.
    fn gen_rook_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let kernel: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut moves: Vec<(usize, usize, Flags)> = vec![];

        for k in kernel.iter() {
            let mut d: (i8, i8) = (index.0 + k.0, index.1 + k.1);

            while self.within_board(d) {
                if self.enemy_tile((d.0 as usize, d.1 as usize), team) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::Capture));
                    break;
                } else if self.empty_tile((d.0 as usize, d.1 as usize)) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::None));
                } else {
                    break;
                }

                d = (d.0 + k.0, d.1 + k.1);
            }
        }

        return moves;
    }

    // Generate knight moves.
    fn gen_knight_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let kernel: [(i8, i8); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (-1, 2), (1, -2), (-1, -2)];
        let mut moves: Vec<(usize, usize, Flags)> = vec![];
        
        for k in kernel.iter() {
            let d: (i8, i8) = (index.0 + k.0, index.1 + k.1);
            if self.within_board(d) {
                if self.enemy_tile((d.0 as usize, d.1 as usize), team) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::Capture));
                } else if self.empty_tile((d.0 as usize, d.1 as usize)) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::None));
                }
            }
        }

        return moves;
    }

    // Generate bishop moves.
    fn gen_bishop_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let kernel: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
        let mut moves: Vec<(usize, usize, Flags)> = vec![];

        for k in kernel.iter() {
            let mut d: (i8, i8) = (index.0 + k.0, index.1 + k.1);

            while self.within_board(d) {
                if self.enemy_tile((d.0 as usize, d.1 as usize), team) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::Capture));
                    break;
                } else if self.empty_tile((d.0 as usize, d.1 as usize)) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::None));
                } else {
                    break;
                }

                d = (d.0 + k.0, d.1 + k.1);
            }
        }

        return moves;
    }

    // Generate queen moves.
    fn gen_queen_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let mut moves: Vec<(usize, usize, Flags)> = vec![];
        moves.append(&mut self.gen_rook_move(index, team));
        moves.append(&mut self.gen_bishop_move(index, team));

        return moves;
    }

    // Generate king moves.
    fn gen_king_move(&self, index: (i8, i8), team: i8) -> Vec<(usize, usize, Flags)> {
        let kernel: [(i8, i8); 8] = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (1, 1), (0, 1), (-1, 1)];
        let mut moves: Vec<(usize, usize, Flags)> = vec![];

        for k in kernel.iter() {
            let d: (i8, i8) = (index.0 + k.0, index.1 + k.1);

            if self.within_board(d) {
                if self.enemy_tile((d.0 as usize, d.1 as usize), team) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::Capture));
                } else if self.empty_tile((d.0 as usize, d.1 as usize)) {
                    moves.push((d.0 as usize, d.1 as usize, Flags::None));
                }
            }
        }

        let r: usize = if team == -1 { 7 } else { 0 };
        if r == 7 {
            if self.wqcr && self.empty_tile((1, r)) && self.empty_tile((2, r)) && self.empty_tile((3, r)) { moves.push((2, r, Flags::Qastling)); } 
            if self.wkcr && self.empty_tile((5, r)) && self.empty_tile((6, r)) { moves.push((6, r, Flags::Kastling)); }
        } else {
            if self.bqcr && self.empty_tile((1, r)) && self.empty_tile((2, r)) && self.empty_tile((3, r)) { moves.push((2, r, Flags::Qastling)); } 
            if self.bkcr && self.empty_tile((5, r)) && self.empty_tile((6, r)) { moves.push((6, r, Flags::Kastling)); }
        }

        return moves;
    }

    /// Check if tile is empty.
    fn empty_tile(&self, indices: (usize, usize)) -> bool { return self.board[indices.1][indices.0].id == 0; }

    /// Check if tile is enemy tile.
    fn enemy_tile(&self, indices: (usize, usize), team: i8) -> bool { return self.board[indices.1][indices.0].team == -team; }

    /// Check if indices are within board bounds.
    fn within_board(&self, indices: (i8, i8)) -> bool { return indices.0 < 8 && indices.0 > -1 && indices.1 < 8 && indices.1 > -1 }

    /// Print the board to the terminal.
    pub fn print(&self) {
        for y in 0..8usize {
            for x in 0..8usize {
                let col = if self.board[y][x].team == -1 { "32;49" } else { "31;49" };
                print!("\x1b[{}m{}\x1b[0m ", col,
                    match self.board[y][x].id {
                        1 => { "P" }
                        2 => { "R" }
                        3 => { "k" }
                        4 => { "B" }
                        5 => { "Q" }
                        6 => { "K" }
                        _ => { " " }
                    }
                );
            }
            print!("\n");
        }
        print!("\n\n");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}