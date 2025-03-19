mod board;
mod piece;

fn main() {
    let main_board = board::Board::new();

    for row in main_board.squares.into_iter(){
        for square in row {
            square.print();
        }
    }
    
    println!("This is the end of the main function");
}
