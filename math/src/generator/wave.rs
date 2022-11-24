/*
* a board has a vector 2d of cells that can be in two states a collapsed one where 
* it holds a usize a uncollapsed one that holds vector of usizes, the usizes are the
* index of a vector that contains all possible values that the cell could be,
* the vector is owned by the board it also holds the size of
* the units just to no make it annoying to retrive
*
* a cell can be in two states: 
* - collapsed: where the cell contains the index of the unit that is like
* - uncollapsed: where the cell contains all the possible units that it can be
* a unit has 4 borders that can have n amounts of states
*
* units are like this
*
*     a b 
*   |-----|
* b |     | a
* a |     | b
*   |-----|
*     b a 
*
* for example
*
* lets say there is 4 types of borders of 2 bits long
*     ab
* AIR 00 where there is only air
* RIG 01 where there is only solid in the right
* LEF 10 where there is only solid in the left
* SOL 11 where there is only solid
*
* a fully air unit would be like
* n = AIR, s = RIG, w = AIR, e = AIR
*
* a surface unit would be
* n = AIR, s = SOL, w = LEF, e = RIG
*
* a border of two surface uints are 
*
*     0 0       0 1
*   |-----|   |-----|
* 0 | uni | 0 | uni | 0
* 1 |  1  | 1 |  2  | 1
*   |-----|   |-----|
*     1 1       1 1
*
* for uni_1 it's east is 01 
* for uni_2 its' west is 10
*
* and to evaluate that they share a border you need to
* get the opposite of the value of one two make them match and then being
* able to say "yes this two match"
*
* the idea of that each border has to be 2 bits is just 
* a easy representation of how they should be paired
*
*
* the border always must have an opposite border 
*/

use {
    super::random::Random,
    super::super::{
        general::{xdia, ydia},
        transform2d::Opposite,
    },
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Unit<T: Default + Eq + PartialEq + Copy + Opposite>{
    pub north: T,
    pub south: T,
    pub east:  T,
    pub west:  T,
}

type Possible<T> = Vec<Unit<T>>;

impl<T: Default + Eq + PartialEq + Copy + Opposite> Unit<T>{
    #[allow(dead_code)]
    pub const fn new(north: T, south: T, east: T, west: T) -> Unit<T>{
        Unit{north, south, east, west}
    }

    #[allow(dead_code)]
    pub const fn rotate(&self, mut degree: usize) -> Unit<T>{
        degree %= 4;
        match degree{
            1 => {Unit::<T>::new(self.east , self.west , self.south, self.north)},
            2 => {Unit::<T>::new(self.south, self.north, self.west , self.east )},
            3 => {Unit::<T>::new(self.west , self.east , self.north, self.south)},
            _ => {Unit::<T>::new(self.north, self.south, self.east , self.west )},
        }
    }

    #[allow(dead_code)]
    pub fn ymirror(&self) -> Unit<T>{
        Unit::<T>::new(self.north.opposite(), self.south.opposite(), self.west , self.east )
    }

    #[allow(dead_code)]
    pub fn xmirror(&self) -> Unit<T>{
        Unit::<T>::new(self.south, self.north, self.west.opposite() , self.east.opposite() )
    }
}

impl<T: Default + Eq + PartialEq + Copy + Opposite> Default for Unit<T>{
    fn default() -> Unit<T> {
        Unit{
            north:  T::default(),
            south:  T::default(),
            east :  T::default(),
            west :  T::default(),
        }
    }
}

type Collapsed = usize;
type Uncollapsed = Vec<usize>;

#[derive(Clone)]
pub enum Cell{
    Collapsed(Collapsed),
    Uncollapsed(Uncollapsed),
}

macro_rules! compare_borders {
    ($possible: tt, $pos: tt, $found: tt, $borderA: tt, $borderB: tt) => {
        $found = false;
        for v in $borderA.uncollapse(){
            if $possible[$pos].$borderA == $possible[v].$borderB.opposite(){
                $found = true;
                break;
            }
        }
        if !$found {continue;}
        
    };
}

impl Cell{
    /*
    * when it is 1 the cell is collapsed
    * when it is different from one the cell in undetermined
    * and the value is all the possible states that it has
    */
    fn entropy(&self) -> usize{
        match self {
            Cell::Collapsed(_) => 1,
            /*
            * this part should really collapse the cell if the vector only holds
            * one value, but I don't think that will ever happen
            * so I don't care
            */
            Cell::Uncollapsed(v) => v.len(),
            
        }
    }

    fn collapsed(&self) -> bool{
        matches!(&self, Cell::Collapsed(_))
    }

    /*
    * this function should be only used once you
    * verified the state of the cell
    */
    pub fn collapse_val(&self) -> usize{
        match &self {
            Cell::Collapsed(ind) => ind.clone(),
            _ => usize::MAX
        }
    }

    fn uncollapse(&self) -> Vec<usize>{
        match self {
            Self::Uncollapsed(u) => u.clone(), 
            Self::Collapsed(c) => vec![c.clone()],
        }
    }

    /* this is going to have a long ass documentation lmao
    *
    * it takes as it's inputs the bordering cells,
    * if there is no bordering cell it should take a default cell
    * all the possible cells it can be
    * and returns its entropy
    */

    fn collapse<BorderT: Eq + PartialEq + Default + Copy + Opposite>(
        &mut self,
        north: &Cell, south: &Cell, east: &Cell, west: &Cell,
        possible: &Possible<BorderT>
    ) -> usize
    {
        if self.collapsed(){
            return 1;
        }
        let should_collapse = north.collapsed() && south.collapsed() && east.collapsed() && west.collapsed();

        //should it have a single escape part idk i can't care 
        if should_collapse {
            let expected = Unit::new(
                possible[north.collapse_val()].south,
                possible[south.collapse_val()].north,
                possible[east .collapse_val()].west ,
                possible[west .collapse_val()].east ,
            );

            let mut index = 0;
            for p in possible{
                if &expected == p{
                    *self = Cell::Collapsed(index);
                    break;
                }
                index += 1;
            }
        }
        else{
            let current = self.uncollapse();
            let mut new_self = Uncollapsed::new();
            for pos in current{
                let mut found: bool;
                compare_borders!(possible, pos, found, north, south);
                compare_borders!(possible, pos, found, south, north);
                compare_borders!(possible, pos, found, east , west );
                compare_borders!(possible, pos, found, west , east );

                new_self.push(pos);
            }
            if new_self.len() == 1 {
                *self = Cell::Collapsed(new_self[0]);
            }
            else{
                *self = Cell::Uncollapsed(new_self);
            }
        }
        return self.entropy();
    }

    pub fn force_collapse(&mut self, seed: u64){
        let coll = match self {
            Cell::Collapsed(_) => return,
            Cell::Uncollapsed(u) => u[usize::rands_range(0, u.len(), seed)],
        };

        *self = Cell::Collapsed(coll);
    }
}

pub struct FiniteMap<BorderT: Default + Eq + PartialEq + Copy + Opposite>{
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

impl<BorderT: Eq + PartialEq + Default + Copy + Opposite>  FiniteMap<BorderT>{
    pub fn new(width: usize, height: usize, possible: Possible<BorderT>, seed: u64,) -> FiniteMap<BorderT>{
        let mut map: Vec<Vec<Cell>> = Vec::<Vec<Cell>>::new();
        let mut default_vec: Vec::<Cell> = Vec::<Cell>::new();
        let mut default: Vec<usize> = Vec::new();
        for i in 0..possible.len(){
            default.push(i);
        }
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
        let entropy = self.possible.len();
        if north.entropy() == entropy && south.entropy() == entropy && east.entropy() == entropy && west.entropy() == entropy {
            return false;
        }
        else{
            // wacky shit bc I still don't understand the 
            // borrow checker lmao
            let mut cell = self.map[i][j].clone();
            cell.collapse(north, south, east, west, &self.possible);
            self.map[i][j].clone_from(&cell);
            return self.map[i][j].entropy() != self.possible.len();
        }
    }

    #[allow(dead_code)]
    pub fn print_self(&self){
        for __j in 0..self.height{
            let j = (self.width - 1) - __j;
            print!("{}: ", j);
            for i in 0..self.width{
                match &self.map[i][j] {
                    Cell::Collapsed(c) => print!("|{}", c),
                    Cell::Uncollapsed(u) => {
                        //print!("[{}", u.len());
                        print!("[");
                        for i in u{
                            print!("{},", i);
                        }
                        print!("");
                    }
                }
            }
            println!();
        }
        println!();
    } 

    pub fn cirular_collapse(&mut self, i: usize, j: usize){
        for r in 0..=(self.width + self.height){
            for step in 1..= 4 * r{
                let ni = i as i64 + xdia(step as u64, r as u64);
                let nj = j as i64 + ydia(step as u64, r as u64);

                if ni < 0 || ni >= self.width as i64 || nj < 0 || nj >= self.height as i64 {
                    continue;
                }

                self.collapse_cell(ni as usize, nj as usize);
            }
        }
    }

    pub fn force_collapse(&mut self, i: usize, j: usize){
        let seed = self.seed + (i + j * self.height) as u64;
        self.map[i][j].force_collapse(seed);
    }

    pub fn determine(&mut self) {
        let ci = usize::rands_range(0, self.width,  self.seed);
        let cj = usize::rands_range(0, self.height, self.seed + ci as u64);

        self.force_collapse(ci, cj);
        self.cirular_collapse(ci, cj);

        loop {
            let v = self.least();
            if v.vec.len() == 0{
                break;
            }
            let cp = v.vec[usize::rand_range(0, v.vec.len())];
            self.force_collapse(cp[0], cp[1]);
            self.cirular_collapse(cp[0], cp[1]);
        }
    }

    pub fn least(&self) -> LeastContainer {
        let mut vec = Vec::<[usize; 2]>::new();

        let mut min_grade = self.possible.len();
        for i in 0..self.width{
            for j in 0..self.height{
                let grade = self.map[i][j].entropy();
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
