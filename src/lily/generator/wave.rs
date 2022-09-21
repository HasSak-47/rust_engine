use std::collections::HashMap;

use crate::lily::generator::base::Generator;

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq)]
struct Unit<T: Eq + Clone + Default>{
    pub north: T,
    pub south: T,
    pub east : T,
    pub west : T,
}

#[derive(Hash, Eq, PartialEq)]
enum Cell<T: Eq + Clone + Default>{
    Collapsed(Unit<T>),
    UnCollapsed(Vec<Unit<T>>),
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord{
    fn new(x: i64, y: i64,) -> Coord{
        Coord{x, y}
    }
}

pub struct Map<T: Eq + Clone + Default> {
    pvals: Vec<Unit<T>>, 
    board: HashMap<Coord, usize>,
}

struct Neightbors {
    north : usize,
    south : usize,
    west  : usize,
    east  : usize,
    center: usize,
}

fn collapse<T: Eq + Clone + Default>(x: i64, y: i64,map: &mut Map<T>) {
    if map.board.contains_key(&Coord{x, y}){

    }
}

impl<T: Eq + Clone + Default> Generator<Unit<T>, i64> for Map<T>{
    fn generate_2d(self, x: i64, y: i64) -> Unit<T> {
        self.pvals.get(0).unwrap().clone()
    }
    fn generate_3d(self, x: i64, y: i64, z: i64) -> Unit<T> {
        self.pvals.get(0).unwrap().clone()
    }

}
