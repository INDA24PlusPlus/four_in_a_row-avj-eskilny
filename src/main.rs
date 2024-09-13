use std::io;

#[derive(Copy, Clone, PartialEq)]
enum Color {
    RED,
    YELLOW,
}

fn drop_coin(col: i32, color: Color, board: &mut [[Option<Color>; 7]; 6]) {
    for i in 0..5 {
        if i < 5 && !board[(i + 1) as usize][col as usize].is_none() {
            board[i as usize][col as usize] = Some(color);
            break;
        }
    }
}

fn four_in_a_row(color: Color, board: &mut [[Option<Color>; 7]; 6]) -> bool {
    // check up-down, sidewards and diagonal

    // check up

    let has_won = false;
    for i in 0..6 {
        let mut streak = 0;
        for j in 0..5 {
            if streak == 4 {
                return true;
            }
            if board[i as usize][j as usize].is_some()
                && board[i as usize][j as usize].unwrap() == color
            {
                streak += 1;
            } else {
                streak = 0;
            }
        }
    }

    // check sideways
    for i in 0..5 {
        let mut streak = 0;
        for j in 0..6 {
            if streak == 4 {
                return true;
            }
            if board[j as usize][i as usize].is_some()
                && board[j as usize][i as usize].unwrap() == color
            {
                streak += 1;
            } else {
                streak = 0;
            }
        }
    }

    // check diagonals
    for i in 0..5 {
        for j in 0..6 {
            let streak = 0;
            // snett uppåt höger
            for a in j..7 {}

            // snett neråt (-x, -y)
            for a in (j..0).rev() {}
        }
    }

    return has_won;
}

fn main() {
    let mut board: [[Option<Color>; 7]; 6] = [[None; 7]; 6];
    println!("Welcome to Four in a Row!\nShould RED or YELLOW start?");
    let mut starting_color = read_line();

    /*  while !starting_color.to_uppercase().eq("RED") && !starting_color.to_uppercase().eq("YELLOW") {
        println!("Try again!");
        starting_color = read_line();
    } */
    println!("Got it! {} starts.", starting_color.to_ascii_uppercase());

    let mut game_over = false;
    let current_color = "RED"; // TODO update colour in library
    while !game_over {
        println!("It is currently {}'s turn.", current_color);
        // TODO println!({}, board)
        let mut output = String::new();

        for row in 0..6 {
            for col in 0..5 {
                output.push(match board[row][col] {
                    Some(p) => {
                        if p == Color::RED {
                            'R'
                        } else {
                            'Y'
                        }
                    }
                    None => '0',
                });
            }
            output.push_str("\n");
        }

        // end with the bottom rank
        output.push_str("1234567");
        println!("Choose a column (1-7) to place the piece in!");
        let mut col_str = read_line();
        let mut col_res = col_str.trim().parse::<u8>();
        let mut ok = match col_res {
            Ok(col) => col > 0 && col < 8,
            Err(_) => false,
        };
        while !ok {
            println!("Try again!");
            col_str = read_line();
            col_res = col_str.trim().parse::<u8>();
            ok = match col_res {
                Ok(col) => col > 0 && col < 8,
                Err(_) => false,
            };
            if ok {
                // TODO is column full?
                // if full
                ok = false;
            }
        }
        let col = col_res.unwrap();
        let color_e: Option<Color> = match current_color {
            "RED" => Some(Color::RED),
            "YELLOW" => Some(Color::YELLOW),
            _ => None,
        };
        // TODO perform move
        drop_coin(col as i32, color_e.unwrap(), &mut board);

        if four_in_a_row(color_e.unwrap(), &mut board) {
            game_over = true;
        }
    }
}

fn read_line() -> String {
    let mut output = String::new();
    io::stdin().read_line(&mut output).expect("");
    return output;
}
