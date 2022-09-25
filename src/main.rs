mod lily;
use crate::lily::generator::wave::{
    Unit, Cell
};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tborder{
    Rig = 0b01,
    Lef = 0b10,
    Tot = 0b11,
    Non = 0b00,
}

// to analyze the side it has to be like this
//   a b
// a|--|a
// b|__|b
//  a b

type Tunit = Unit<Tborder>;

fn main(){
    let units = vec![
        Tunit::new( // air
            Tborder::Non,
            Tborder::Non,
            Tborder::Non,
            Tborder::Non,
        ),
        Tunit::new( // ground
            Tborder::Non,
            Tborder::Tot,
            Tborder::Rig,
            Tborder::Rig,
        ),
        Tunit::new( // solid
            Tborder::Tot,
            Tborder::Tot,
            Tborder::Tot,
            Tborder::Tot,
        ),
    ];

    let mut test_board = [
        [Cell::new_unc(0, 3), Cell::new_unc(0, 3), Cell::new_unc(0,3)],
        [Cell::new_unc(0, 3), Cell::Collapsed(1) , Cell::new_unc(0,3)],
        [Cell::new_unc(0, 3), Cell::new_unc(0, 3), Cell::new_unc(0,3)],
    ];

    let def = Cell::new_unc(0, 3);
    
    for i in 0..3{
        for j in 0..3{
            let north = match i {
                2 => def.clone(),
                _ => test_board[i + 1][j].clone(),
            };
            let south = match i {
                0 => def.clone(),
                _ => test_board[i - 1][j].clone(),
            };
            let east = match j {
                2 => def.clone(),
                _ => test_board[i][j + 1].clone(),
            };
            let west = match j {
                0 => def.clone(),
                _ => test_board[i][j - 1].clone(),
            };
            test_board[i][j].collapse(&north, &south, &east, &west, &units);
        }
    }

    for i in 0..3{
        for j in 0..3{
            //print!("({}, {}): ", i, j);
            match &test_board[i][j]{
                Cell::Collapsed(x) =>
                    print!("{}", x),
                Cell::Uncollapsed(_) =>
                    print!("o"),
            }
        }
        println!();
    }
}
