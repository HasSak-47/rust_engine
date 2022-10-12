
pub fn diamond(input: u64, rad: u64) -> i64{
    let i = input.clone() as i64;
    let r = rad.clone() as i64;
    (i % (4 * r) - 2 * r).abs() - r
}

pub fn xdia(x: u64, rad: u64) -> i64{
    diamond(x, rad)
}

pub fn ydia(x: u64, rad: u64) -> i64{
    diamond(x + 3 * rad, rad)
}
