use crate::lily::{
    generator::{
        base::{self, Seed64},
        random::*,
    },
    bindings::c_perlin,
};

pub struct PerlinGen2D{
    pub seed: Seed64,
    pub x_wrap: u64,
    pub y_wrap: u64,
    pub z_wrap: u64,
}


