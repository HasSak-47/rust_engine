mod lily;
use lily::generator::random::set_global_seed;

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
    set_global_seed(&10);
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
        Tunit::new( // left dropoff the solid is in the left
            Tborder::Non,
            Tborder::Rig,
            Tborder::Non,
            Tborder::Rig,
        ),
        Tunit::new( // right dropoff
            Tborder::Non,
            Tborder::Lef,
            Tborder::Lef,
            Tborder::Non,
        ),
        Tunit::new( // left cliff
            Tborder::Rig,
            Tborder::Rig,
            Tborder::Non,
            Tborder::Tot,
        ),
        Tunit::new( // right cliff
            Tborder::Lef,
            Tborder::Lef,
            Tborder::Tot,
            Tborder::Non,
        ),
    ];

    //let mut test_board = [
    //    [Cell::new_unc(0, 3), Cell::new_unc(0, 3), Cell::new_unc(0,3)],
    //    [Cell::new_unc(0, 3), Cell::Collapsed(1) , Cell::new_unc(0,3)],
    //    [Cell::new_unc(0, 3), Cell::new_unc(0, 3), Cell::new_unc(0,3)],
    //];
    
    let mut test_board: [[Cell; 10]; 10] = Default::default();
    let def = Cell::new_unc(0, units.len());
    for i in 0..100 {
        let x = i / 10;
        let y = i % 10;
        test_board[x][y] = def.clone();
    }

    test_board[5][5] = Cell::Collapsed(1);

    for i in 0..10{
        for j in 0..10{
            let north = match i {
                9 => def.clone(),
                _ => test_board[i + 1][j].clone(),
            };
            let south = match i {
                0 => def.clone(),
                _ => test_board[i - 1][j].clone(),
            };
            let east = match j {
                9 => def.clone(),
                _ => test_board[i][j + 1].clone(),
            };
            let west = match j {
                0 => def.clone(),
                _ => test_board[i][j - 1].clone(),
            };
            let cell = &mut test_board[i][j];
            cell.collapse(&north, &south, &east, &west, &units);
            //if cell.size() != 7 {
            //    cell.force_collapse();
            //}
        }
    }

    for __i in 0..10{
        for j in 0..10{
            let i = 9 - __i;
            match &test_board[i][j]{
                Cell::Collapsed(c) =>
                    print!("{}", c),
                Cell::Uncollapsed(u) => {
                    print!("o");
                }
            }
        }
        println!();
    }
}
