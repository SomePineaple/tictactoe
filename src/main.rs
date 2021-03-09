mod engine;

fn main() {
    let mut engine = engine::Engine {
        is_x_turn: true,
        board: [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']],
    };

    engine.print_board();
    while !engine.is_game_over() {
        engine.get_user_input();
        engine.make_computer_move();
        engine.print_board();
    }

    match engine.get_game_status() {
        engine::GameStatus::XWINS => {
            println!("x wins")
        }
        engine::GameStatus::OWINS => {
            println!("o wins");
        }
        engine::GameStatus::CATSGAME => {
            println!("Cats game");
        }
        engine::GameStatus::NOTOVER => {}
    }
}
