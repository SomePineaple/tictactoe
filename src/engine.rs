use std::io::stdin;
use std::io::stdout;
use std::io::Write;

pub struct Engine {
    pub board: [[char; 3]; 3],
    pub is_x_turn: bool,
}

struct BoardPos {
    x: u8,
    y: u8,
}

pub enum GameStatus {
    XWINS,
    OWINS,
    CATSGAME,
    NOTOVER,
}

impl Engine {
    pub fn print_board(&self) {
        for y in 0..3 {
            for x in 0..3 {
                print!("{}", self.board[y][x]);
                if x != 2 {
                    print!(" | ");
                }
            }
            println!();
            if y != 2 {
                println!("---------",);
            }
        }
    }

    pub fn get_user_input(&mut self) {
        let x = get_input("Enter an x position for your move: ");
        let y = get_input("Enter a y position for your move: ");

        let xi = x.parse::<u8>().unwrap();
        let yi = y.parse::<u8>().unwrap();
        if !self.make_move(&BoardPos { x: xi, y: 2 - yi }) {
            println!("Invalid board position, try again");
            self.get_user_input();
        }
    }

    pub fn make_computer_move(&mut self) {
        if self.is_game_over() {
            return;
        }
        let mut highest_val = -9999;
        let mut lowest_val = 9999;
        let mut best_pos = BoardPos { x: 9, y: 9 };

        for pos in self.get_positions() {
            self.make_move(&pos);
            let current_val: i32;
            if self.is_x_turn {
                current_val = self.minimax(true);
            } else {
                current_val = self.minimax(false);
            }
            self.undo_move(&pos);
            if self.is_x_turn && current_val > highest_val {
                highest_val = current_val;
                best_pos = pos;
                if current_val == 100 {
                    break;
                }
            } else if !self.is_x_turn && current_val < lowest_val {
                lowest_val = current_val;
                best_pos = pos;
                if current_val == 100 {
                    break;
                }
            }
        }

        if best_pos.x != 9 {
            self.make_move(&best_pos);
            println!("Move value: {}", lowest_val);
        } else {
            println!("Computer failed to find move");
        }
    }

    fn minimax(&mut self, is_maximizing: bool) -> i32 {
        if self.is_game_over() {
            match self.get_game_status() {
                GameStatus::XWINS => {
                    return 1;
                }
                GameStatus::OWINS => {
                    return -1;
                }
                GameStatus::CATSGAME => {
                    return 0;
                }
                GameStatus::NOTOVER => {}
            }
        }

        let mut highest_val = -9999;
        let mut lowest_val = 9999;

        for pos in self.get_positions() {
            self.make_move(&pos);
            let current_val = self.minimax(!is_maximizing);
            self.undo_move(&pos);
            if is_maximizing && current_val > highest_val {
                highest_val = current_val;
            }
            if !is_maximizing && current_val < lowest_val {
                lowest_val = current_val;
            }
        }

        if is_maximizing {
            return highest_val;
        }
        return lowest_val;
    }

    fn get_positions(&self) -> Vec<BoardPos> {
        let mut positions = Vec::new();

        for y in 0..3 {
            for x in 0..3 {
                if self.board[y][x] == ' ' {
                    positions.push(BoardPos {
                        x: x as u8,
                        y: y as u8,
                    });
                }
            }
        }

        return positions;
    }

    pub fn is_game_over(&self) -> bool {
        if let GameStatus::NOTOVER = self.get_game_status() {
            return false;
        }
        return true;
    }

    pub fn get_game_status(&self) -> GameStatus {
        let mut cats_game = true;

        let mut is_all_x: bool;
        let mut is_all_o: bool;

        for row in 0..3 {
            is_all_x = true;
            is_all_o = true;

            for space in 0..3 {
                if self.board[row][space] != 'x' {
                    is_all_x = false
                }
                if self.board[row][space] != 'o' {
                    is_all_o = false;
                }
                if self.board[row][space] == ' ' {
                    cats_game = false;
                }
            }
            if is_all_x {
                return GameStatus::XWINS;
            }

            if is_all_o {
                return GameStatus::OWINS;
            }
        }

        for col in 0..3 {
            is_all_x = true;
            is_all_o = true;
            for space in 0..3 {
                if self.board[space][col] != 'x' {
                    is_all_x = false;
                }
                if self.board[space][col] != 'o' {
                    is_all_o = false;
                }
                if self.board[space][col] == ' ' {
                    cats_game = false;
                }
            }
            if is_all_x {
                return GameStatus::XWINS;
            }
            if is_all_o {
                return GameStatus::OWINS;
            }
        }

        let diag1 = [
            BoardPos { x: 0, y: 0 },
            BoardPos { x: 1, y: 1 },
            BoardPos { x: 2, y: 2 },
        ];

        let diag2 = [
            BoardPos { x: 0, y: 2 },
            BoardPos { x: 1, y: 1 },
            BoardPos { x: 2, y: 0 },
        ];

        is_all_x = true;
        is_all_o = true;

        diag1.iter().for_each(|pos| {
            if self.board[pos.y as usize][pos.x as usize] != 'x' {
                is_all_x = false;
            }
            if self.board[pos.y as usize][pos.x as usize] != 'o' {
                is_all_o = false;
            }
        });

        if is_all_x {
            return GameStatus::XWINS;
        }
        if is_all_o {
            return GameStatus::OWINS;
        }

        is_all_x = true;
        is_all_o = true;

        diag2.iter().for_each(|pos| {
            if self.board[pos.y as usize][pos.x as usize] != 'x' {
                is_all_x = false;
            }
            if self.board[pos.y as usize][pos.x as usize] != 'o' {
                is_all_o = false;
            }
        });

        if is_all_x {
            return GameStatus::XWINS;
        }
        if is_all_o {
            return GameStatus::OWINS;
        }

        if cats_game {
            return GameStatus::CATSGAME;
        }
        return GameStatus::NOTOVER;
    }

    fn make_move(&mut self, pos: &BoardPos) -> bool {
        if self.board[pos.y as usize][pos.x as usize] != ' ' {
            return false;
        } else {
            if self.is_x_turn {
                self.board[pos.y as usize][pos.x as usize] = 'x';
            } else {
                self.board[pos.y as usize][pos.x as usize] = 'o';
            }
            self.is_x_turn = !self.is_x_turn;
            return true;
        }
    }

    fn undo_move(&mut self, pos: &BoardPos) {
        self.board[pos.y as usize][pos.x as usize] = ' ';
        self.is_x_turn = !self.is_x_turn;
    }
}

fn get_input(prompt: &str) -> String {
    let mut input_str = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_str)
        .expect("Did not enter a correct string");
    if let Some('\n') = input_str.chars().next_back() {
        input_str.pop();
    }
    if let Some('\r') = input_str.chars().next_back() {
        input_str.pop();
    }
    return input_str;
}
