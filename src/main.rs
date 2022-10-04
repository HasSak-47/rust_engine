mod lily;

use crossterm::event::PushKeyboardEnhancementFlags;
use lily::generator::wave::*;
use lily::generator::random::*;
use lily::general::*;
use std::time::{UNIX_EPOCH, SystemTime, Duration};
use std::fmt;

 #[derive(Copy, Clone, Eq, PartialEq, Default)]
 enum Tborder{
     Rig = 0b01,
     Lef = 0b10,
     Tot = 0b11,
     #[default]
     Non = 0b00,
 }

impl Mirror for Tborder{
    fn mirror(&self) -> Self {
        match self {
            Tborder::Rig => Tborder::Lef,
            Tborder::Lef => Tborder::Rig,
            Tborder::Tot => Tborder::Tot,
            Tborder::Non => Tborder::Non,
        }
    }
}

fn fmt(a: &Tborder) -> &str {
    match a{
        Tborder::Non => "0b00",
        Tborder::Rig => "0b01",
        Tborder::Lef => "0b10",
        Tborder::Tot => "0b11",
    }
}

fn main(){
    let mut units = vec![
        Unit { // air
            north: Tborder::Non,
            south: Tborder::Non, //
            east:  Tborder::Non, //
            west:  Tborder::Non,
        },
        Unit { // surface
            north: Tborder::Non,
            south: Tborder::Tot, // __ 
            east:  Tborder::Rig, // ##
            west:  Tborder::Lef,
        },
        Unit { // ground
            north: Tborder::Tot,
            south: Tborder::Tot, // ## 
            east:  Tborder::Tot, // ## 
            west:  Tborder::Tot,
        },
        Unit { //edge
            north: Tborder::Non,
            south: Tborder::Lef, //  -
            east:  Tborder::Rig, // |#
            west:  Tborder::Non,
        },
    ];

    
    units.push(units[1].rotate(1));
    units.push(units[1].rotate(2));
    units.push(units[1].rotate(3));
    units.push(units[3].rotate(1));
    units.push(units[3].rotate(2));
    units.push(units[3].rotate(3));
    let symbols = [
        ["  ",
         "  "], // 0
        ["--",
         "##"], // 1
        ["##",
         "##"], // 2
        ["|-",
         "|#"], // 3

        ["|#",
         "|#"], // 4
        ["##",
         "--"], // 5
        ["#|",
         "#|"], // 6
        ["|#",
         "|-"], // 7
        ["#|",
         "-|"], // 8
        ["-|",
         "#|"], // 9
    ];

    for unit in &units{
        println!("{}", fmt(&unit.north));
        println!("{}", fmt(&unit.south));
        println!("{}", fmt(&unit.east ));
        println!("{}\n", fmt(&unit.west ));
    }
    
    //let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis() as u64;
    let now = 1664905806579;
    println!("{}", now);

    for i in 0..2{
        for j in 0..(&units.len()).clone(){
            print!("1 {} 1", symbols[j][i]);
        }
        println!();
    }

    let mut test_board = FiniteMap::<Tborder>::new(
        10,
        10,
        units,
        now,
    );

    test_board.determine();
    test_board.print_self();

    for __y in 0..test_board.height{
        let y = test_board.height - (__y + 1);
        for i in 0..=1 {
            for x in 0..test_board.width{
                print!("{}", symbols[test_board.map[x][y].collapse_val()][i]);
            }
            println!();
        }
    }
    test_board.print_self();
}
