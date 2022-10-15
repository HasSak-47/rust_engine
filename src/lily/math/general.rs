use super::quaternion::Quat;
use super::generator::base::Generator;

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

/*
* really slow implementation lmao
*/
pub fn slope(g: &dyn Generator<f64, f64>, x: f64, y: f64, delta: f64) -> f64{
    let xt = [x - delta, x + delta , x + delta];
    let yt = [y, y - delta , y + delta];

    let r = [
    	g.generate_2d(xt[0], yt[0]),
    	g.generate_2d(xt[1], yt[1]),
    	g.generate_2d(xt[2], yt[2]),
    ];

    let tri = [
        Quat::new(0.0, xt[0], yt[0], r[0]),
        Quat::new(0.0, xt[1], yt[1], r[1]),
        Quat::new(0.0, xt[2], yt[2], r[2]),
    ];

    let mid = [
        tri[2] - tri[0],
        tri[1] - tri[0],
    ];

    let mut result = mid[0] * mid[1];
    result.r = 0.0;
    let z = -result.k / result.abs(); 
    let a = 1.0 * z.acos() / std::f64::consts::PI;

    a
}
