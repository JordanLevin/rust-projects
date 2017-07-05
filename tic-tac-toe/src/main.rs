use std::io;

fn main() {
    let mut board: [[char; 3]; 3] = [['-'; 3]; 3];
    let mut player = 0;
    loop {
        print(board);
        player = player%2;
        let mut x = String::new();
        let mut y = String::new(); 
        println!("Player {} enter coordinate 1", player+1);
        io::stdin().read_line(&mut x).expect("Failed to read input");
        let x: usize = match x.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; 
        println!("Player {} enter coordinate 2", player+1);
        io::stdin().read_line(&mut y).expect("Failed to read input");
        let y: usize = match y.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        board[x][y] = if player == 1{
            'X'
        } else {
            'O'
        };
        player+=1;

    }
}

fn print(board: [[char; 3]; 3]){
    for row in board.iter(){
        for item in row.iter(){
            print!("{}",item);
        }
        println!();
    } 
}

fn check_wins(board: [[char; 3]; 3], player: i32) -> bool{
    let c = if player == 1{
        'X'
    } else {
        'O'
    };
    if board[0][0] == c && board[0][1] == c && board[0][2] == c{
        return true;
    }
    if board[1][0] == c && board[1][1] == c && board[1][2] == c{
        return true;
    }
    if board[2][0] == c && board[2][1] == c && board[2][2] == c{
        return true;
    }
    if board[0][0] == c && board[1][0] == c && board[2][0] == c{
        return true;
    }
    if board[0][1] == c && board[1][1] == c && board[2][1] == c{
        return true;
    }
    if board[0][2] == c && board[1][2] == c && board[2][2] == c{
        return true;
    }
    if board[0][0] == c && board[1][1] == c && board[2][2] == c{
        return true;
    }
    if board[0][2] == c && board[1][1] == c && board[2][0] == c{
        return true;
    }
    return false;

}
