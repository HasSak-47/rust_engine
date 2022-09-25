/*
* board: Has a collection of cells
* a cell can be in two states: 
* - collapsed: where the cell contains the index of the unit that is like
* - uncollapsed: where the cell contains all the possible units that it can be
* a unit has 4 borders that can have n amounts of states
*/

use std::vec::Vec;
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

    fn uncollapse(&self) -> Uncollapsed{
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
            // it failed what now
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
}
