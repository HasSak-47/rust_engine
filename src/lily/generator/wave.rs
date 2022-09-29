/*
* board: Has a collection of cells
* a cell can be in two states: 
* - collapsed: where the cell contains the index of the unit that is like
* - uncollapsed: where the cell contains all the possible units that it can be
* a unit has 4 borders that can have n amounts of states
*/

use std::vec::Vec;
use std::io;
use crate::lily::generator::random::Random;

type Collapsed = usize;

type Uncollapsed = Vec<usize>;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Unit<T: Eq + PartialEq + Copy + Clone>{
    pub north: T,
    pub south: T,
    pub east : T,
    pub west : T,
}

#[derive(Clone)]
pub enum Cell{
    Collapsed(Collapsed),
    Uncollapsed(Uncollapsed),
}

#[derive(Eq, PartialEq)]
struct Unknow{
    pub north: Uncollapsed,
    pub south: Uncollapsed,
    pub east : Uncollapsed,
    pub west : Uncollapsed,
}

type Possible<T> = Vec<Unit<T>>;

impl<T: Eq + PartialEq + Copy> Unit<T>{
    pub const fn new(north: T, south: T, east: T, west: T) -> Unit<T>{
        Unit::<T>{north, south, west, east}
    }
}

impl Default for Cell{
    fn default() -> Self {
        Cell::Collapsed(0)
    }
}

impl Cell {
    pub fn new_unc(min: usize, max: usize) -> Cell{
        //idk how to do this lmao
        let mut v = Vec::<usize>::new();
        for i in min..max {
            v.push( i); 
        }
    
        Cell::Uncollapsed(v)
    }
    
    pub fn size(&self) -> usize {
        match self{
            Cell::Collapsed(_) => 1,
            Cell::Uncollapsed(u) => u.len(),
        }
    }

    pub fn uncollapse(&self) -> Uncollapsed{
        match self{
            Cell::Uncollapsed(u) => u.clone(),
            Cell::Collapsed(c) => vec![c.clone()],
        }
    } 

    fn is_collapsed(&self) -> bool{
        match self {
            Cell::Collapsed(_) => true,
            Cell::Uncollapsed(_) => false,
        }
    }

    fn is_collapsed_mut(&mut self) -> bool{
        self.is_collapsed()
    }

    fn collapsed(&self) -> Option<Collapsed> {
        match self{
            Cell::Uncollapsed(_) => None,
            Cell::Collapsed(col) => Some(col.clone()),
        }
    }

    pub fn collapse<BorderT : Eq + PartialEq + Copy>(
        &mut self,
        north: &Cell, south: &Cell, east: &Cell, west: &Cell,
        possible: &Possible<BorderT>
    ){
        match self{
            Cell::Collapsed(_) => return,
            Cell::Uncollapsed(_) => {},
        }
        // check if all borders have a bit of definition

        let collapse = north.is_collapsed() && south.is_collapsed() && east.is_collapsed() && west.is_collapsed();
        // all the borders are defined so write what it expects
        if collapse {
            let expected = Unit::<BorderT> {
                 north: possible[north.collapsed().unwrap_or(0 as usize)].south,
                 south: possible[south.collapsed().unwrap_or(0 as usize)].north,
                 east : possible[ east.collapsed().unwrap_or(0 as usize)].west ,
                 west : possible[ west.collapsed().unwrap_or(0 as usize)].east ,
            };
            let mut i = 0 as usize;
            //search for the border
            for poss in possible{
                if &expected == poss{
                    self.clone_from(&Cell::Collapsed(i));
                    return;
                }
                i += 1;
            }
            // it failed what now?
            // i dont' know
            return;
        }
        let mut new_self = Uncollapsed::new();
        for u in self.uncollapse(){
            let mut found = false;
            for n in north.uncollapse(){
                if possible[n].south == possible[u].north{
                    found = true;
                    break;
                }
            }
            if !found {continue;}
            found = false;
            for s in south.uncollapse(){
                if possible[s].north == possible[u].south{
                    found = true;
                    break;
                }
            }
            if !found {continue;}
            found = false;
            for e in east.uncollapse(){
                if possible[e].west == possible[u].east{
                    found = true;
                    break;
                }
            }
            if !found {continue;}
            found = false;
            for w in west.uncollapse(){
                if possible[w].east == possible[u].west{
                    found = true;
                    break;
                }
            }

            if !found {continue;}
            new_self.push(u);
        }
        if new_self.len() == 1{
            self.clone_from(&Cell::Collapsed(new_self[0]));
        }
        else if new_self.len() > 1{
            self.clone_from(&Cell::Uncollapsed(new_self));
        }
    }

    pub fn force_collapse(&mut self){
        let coll = match self {
            Cell::Collapsed(_) => return,
            Cell::Uncollapsed(u) => u[usize::rand_range(&0, &u.len())],
        };

        self.clone_from(&Cell::Collapsed(coll));
    }

    pub fn force_collapse_s(&mut self, seed: &u64){
        let coll = match self {
            Cell::Collapsed(_) => return,
            Cell::Uncollapsed(u) => u[usize::rands_range(&0, &u.len(), seed)],
        };

        self.clone_from(&Cell::Collapsed(coll));
    }
}

pub struct FiniteMap<BorderT: Copy + Eq + PartialEq>{
    pub width: usize,
    pub height: usize,
    pub default: Uncollapsed,
    pub defcell: Cell,
    pub possible: Possible<BorderT>,
    pub map: Vec<Vec<Cell>>,
    pub seed: u64,
}

pub struct LeastContainer{
    pub vec: Vec::<[usize; 2]>,
    pub grade: usize,
}

impl<BorderT: Copy + Eq + PartialEq + Default>  FiniteMap<BorderT>{
    pub fn new(width: usize, height: usize, default: Uncollapsed, possible: Possible<BorderT>, seed: u64,) -> FiniteMap<BorderT>{
        let mut map: Vec<Vec<Cell>> = Vec::<Vec<Cell>>::new();
        let mut default_vec: Vec::<Cell> = Vec::<Cell>::new();
        default_vec.resize(height, Cell::Uncollapsed(default.clone()));
        map.resize(width, default_vec); 

        FiniteMap{
            width,
            height,
            default: default.clone(),
            defcell : Cell::Uncollapsed(default),
            possible,
            map,
            seed,
        }
    }

    pub fn collapse_cell(&mut self, i: usize, j: usize) -> bool{
        let imax = self.width - 1;
        let jmax = self.height - 1;

        let east = match i {
            p if p >= imax => &self.defcell,
            _ => &self.map[i + 1][j],
        };
        let west = match i {
            0 => &self.defcell,
            _ => &self.map[i - 1][j],
        };
        let north = match j {
            p if p >= jmax => &self.defcell,
            _ => &self.map[i][j + 1],
        };
        let south = match j {
            0 => &self.defcell,
            _ => &self.map[i][j - 1],
        };

        //checks if all the neightbors have some degree of determination
        let size = self.possible.len();
        if north.size() == size && south.size() == size && east.size() == size && west.size() == size {
            return false;
        }
        else{
            // wacky shit bc I still don't understand the 
            // borrow checker lmao
            let mut cell = self.map[i][j].clone();
            cell.collapse(north, south, east, west, &self.possible);
            self.map[i][j].clone_from(&cell);
            return self.map[i][j].size() != self.possible.len();
        }
    }

    pub fn print_self(&self){
        for __j in 0..self.height{
            for i in 0..self.width{
                let j = (self.width - 1) - __j;
                match &self.map[i][j] {
                    Cell::Collapsed(c) => print!("|{}", c),
                    Cell::Uncollapsed(u) => print!("[{}", u.len()),
                }
            }
            println!();
        }
        println!();
    } 

    pub fn cirular_collapse(&mut self, i: usize, j: usize){
        let max = if self.height > self.width {
            self.height
        }
        else{
            self.width
        };
        for r in 1..max{
            let xmax = match i {
                m if m + r >= self.width => self.width,
                _ => i + r,
            };
            let xmin = match i {
                m if m < r => 0,
                _ => i - r,
            };
            let ymax = match j {
                m if m + r >= self.height => self.height,
                _ => j + r,
            };
            let ymin = match j {
                m if m < r => 0,
                _ => j - r,
            };

            for x in xmin..xmax {
                self.collapse_cell(x, j);
            }

            for y in ymin..ymax{
                self.collapse_cell(i, y);
            }
        }
    }

    pub fn determine(&mut self) {
        let ci = usize::rands_range(&0, &self.width, &self.seed);
        let cj = usize::rands_range(&0, &self.height, &self.seed);

        self.map[ci][cj].force_collapse();
        self.cirular_collapse(ci, cj);
        loop {
            let v = self.least();
            if v.vec.len() == 0{
                break;
            }
            for pt in &v.vec{
                self.cirular_collapse(pt[0], pt[1]);
            }
            let cp = v.vec[usize::rand_range(&0, &v.vec.len())];
            self.map[cp[0]][cp[1]].force_collapse_s(&((cp[0] ^ cp[1]) as u64));
            self.cirular_collapse(v.vec[0][0], v.vec[0][1]);
        }
    }

    pub fn least(&self) -> LeastContainer {
        let mut vec = Vec::<[usize; 2]>::new();

        let mut min_grade = self.possible.len();
        for i in 0..self.width{
            for j in 0..self.height{
                let grade = self.map[i][j].size();
                if min_grade > grade && grade > 1{
                    min_grade = grade;
                    vec.clear();
                }
                if min_grade == grade{
                    vec.push([i, j]);
                }
            }
        }

        if min_grade == self.possible.len(){
            vec.clear();
        }
        return LeastContainer {vec, grade: min_grade};
    }
}
