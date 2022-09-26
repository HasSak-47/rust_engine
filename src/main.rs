#[allow(dead_code, unused_imports)]
mod lily;

use lily::generator::{random::set_global_seed, wave::FiniteMap};

use crate::lily::generator::wave::{
    Unit, Cell
};

use std::time::{UNIX_EPOCH, SystemTime, Duration};


#[derive(Copy, Clone, Eq, PartialEq,Default)]
enum Tborder{
    Rig = 0b01,
    Lef = 0b10,
    Tot = 0b11,
    #[default]
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
        Tunit::new( // left dropoff the solid is in the left
            Tborder::Non,
            Tborder::Rig,
            Tborder::Non,
            Tborder::Rig,
        ),
        Tunit::new( // right dropoff
            Tborder::Non,
            Tborder::Lef,
            Tborder::Rig,
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

    let mut vec = Vec::<usize>::new();
    for i in 0..units.len(){
        vec.push(i);
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis();
    let mut test_board = FiniteMap::<Tborder>::new(10, 10, vec, units, now as u64);
    let symbols: [&str; 7]= ["  ", "--", "##", "-|", "|-", "#|", "|#"];
    test_board.determine();
    println!("|----------------------|");
    for __j in 0..test_board.height{
        print!("| ");
        for i in 0..test_board.width{
            let j = test_board.height - (__j + 1);
            let data = symbols[test_board.map[i][j].uncollapse()[0]];
            print!("{}", data);
        }
        println!(" |");
    }
    println!("|----------------------|");
}
