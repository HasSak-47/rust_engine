
pub trait Opposite {
    fn opposite(&self) -> Self;
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Unit<T>{
    pub north: T, 
    pub south: T, 
    pub east : T, 
    pub west : T, 
}

impl<T> Unit<T>{
    pub const fn new(north: T, south: T, east: T, west: T) -> Unit<T>{
        Unit::<T>{north, south, east, west}
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Cell{
    Collapsed(usize),
    Uncollapsed(Vec<usize>),
}

#[derive(Clone, Eq, PartialEq)]
pub struct Map{
    cells: Vec<Cell>,
}


impl Cell{
    pub fn collapsed(&self) -> bool{
        matches!(self, Cell::Collapsed(_))
    }

    pub fn get_possible(&self) -> Vec<usize>{
        match self{
            Cell::Uncollapsed(u) => u.clone(),
            Cell::Collapsed(c) => vec![*c],
        }
    }

    fn remove_north<T>(&mut self, north: &Vec<usize>, possible: &Vec<Unit<T>>) -> &mut Self
    where
        T: Opposite + PartialEq + Eq
    {
        let mut v = Vec::new();
        let self_pos = self.get_possible();
        for nor in north{
            for pos in &self_pos{
                if possible[*pos].north.opposite() == possible[*nor].south {
                    v.push(*pos);
                }
            }
        }

        *self = Cell::Uncollapsed(v);
        self 
    }

    pub fn Collapse<T>(&mut self, north: &Cell, south: &Cell, east: &Cell, west: &Cell, possible: &Vec<Unit<T>>)
    where
        T: Opposite + Eq + PartialEq
    {
        if self.collapsed(){
            return;
        }

        let north = north.get_possible();
        let south = south.get_possible();
        let  east =  east.get_possible();
        let  west =  west.get_possible();

        let self_possible = self.get_possible();


    }
}
