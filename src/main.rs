#[derive(Debug, PartialEq)]
enum Team {
    None,
    White,
    Red
}

#[derive(Debug)]
struct Game {
    board: [[Team; 8]; 8], 
    white_captured: u8,
    red_captured: u8,
    turn: Team,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [
                [Team::None, Team::White, Team::None, Team::White, Team::None, Team::White, Team::None, Team::White],
                [Team::White, Team::None, Team::White, Team::None, Team::White, Team::None, Team::White, Team::None],
                [Team::None, Team::White, Team::None, Team::White, Team::None, Team::White, Team::None, Team::White],
                [Team::None, Team::None, Team::None, Team::None, Team::None, Team::None, Team::None, Team::None],
                [Team::None, Team::None, Team::None, Team::White, Team::None, Team::None, Team::None, Team::None],
                [Team::Red, Team::None, Team::Red, Team::None, Team::Red, Team::None, Team::Red, Team::None],
                [Team::None, Team::Red, Team::None, Team::Red, Team::None, Team::Red, Team::None, Team::Red],
                [Team::Red, Team::None, Team::Red, Team::None, Team::Red, Team::None, Team::Red, Team::None],
            ],
            white_captured: 0,
            red_captured: 0,
            turn: Team::Red,
        }
    }

    fn get_legal_moves(&self, initial: (usize, usize)) -> Result<Vec<(usize, usize)>, &'static str> {
        let initial_pos = self.find_value_of_pos(initial).unwrap(); 

        // Check if value is eq to turn
        if initial_pos.ne(&self.turn) { return Ok(vec![]); };

        let l = self.get_left(initial);
        let r = self.get_right(initial);

        println!("Left: {:?}", l);
        println!("Right: {:?}", r);

        Ok(vec![])
    }

    fn get_left(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = coords;

        if self.turn.eq(&Team::Red) {
            if x == 0 { return None };

            let left_move = self.get_piece((x - 1, y - 1)).unwrap();
            if left_move.eq(&Team::None) { return Some((x - 1, y - 1)) }

            if left_move.eq(&self.turn) { return None }
            if left_move.ne(&self.turn) {
                if x == 1 { return None };

                let left_right = self.get_piece((x - 2, y - 2)).unwrap();
                if left_right.eq(&Team::None) { return Some((x - 2, y - 2)) }
    
                return None
            }
        }

        if self.turn.eq(&Team::White) {
            if x == 0 { return None };

            let left_move = self.get_piece((x + 1, y + 1)).unwrap();
            if left_move.eq(&Team::None) { return Some((x + 1, y + 1)) }

            if left_move.eq(&self.turn) { return None }
            if left_move.ne(&self.turn) {
                if x == 1 { return None };

                let left_right = self.get_piece((x + 2, y + 2)).unwrap();
                if left_right.eq(&Team::None) { return Some((x + 2, y + 2)) }
    
                return None
            }
        }

        None
    }

    fn get_right(&self, coords: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = coords;

        if self.turn.eq(&Team::Red) {
            if x == 0 { return None };

            let right_move = self.get_piece((x + 1, y - 1)).unwrap();
            if right_move.eq(&Team::None) { return Some((x + 1, y - 1)) }

            if right_move.eq(&self.turn) { return None }
            if right_move.ne(&self.turn) {
                if x == 1 { return None };

                let next_right = self.get_piece((x + 2, y - 2)).unwrap();
                if next_right.eq(&Team::None) { return Some((x + 2, y - 2)) }
    
                return None
            }
        }

        if self.turn.eq(&Team::White) {
            if x == 0 { return None };

            let right_move = self.get_piece((x - 1, y + 1)).unwrap();
            if right_move.eq(&Team::None) { return Some((x - 1, y + 1)) }

            if right_move.eq(&self.turn) { return None }
            if right_move.ne(&self.turn) {
                if x == 1 { return None };

                let next_right = self.get_piece((x - 2, y + 2)).unwrap();
                if next_right.eq(&Team::None) { return Some((x - 2, y + 2)) }
    
                return None
            }
        }

        None
    }

    fn get_piece(&self, coords: (usize, usize)) -> Result<&Team, &'static str> {
        let (x, y) = coords;
        
        match self.board.get(y) {
            Some(row) => match row.get(x) {
                Some(pos) => Ok(pos),
                _ => Err("Overflow, x"),
            },
            _ => Err("Overflow, y"),
        }
    }
}

fn main() {
    let game = Game::new();
    let game_move = game.get_legal_moves((4, 5));
}
