use std::{ops::{
    Add, Sub, Mul, Div
}, fmt::format};

#[derive(Debug, Default, Copy, Clone)]
pub struct Quat{
    pub r: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Quat{
    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Quat{
        Quat{r, i, j, k}
    }

    pub fn abs(&self) -> f64{
        ((self.r * self.r) +
        (self.i * self.i) +
        (self.j * self.j) +
        (self.k * self.k)).sqrt()
    }

    pub fn inv(&mut self) -> &mut Self{
        let a = 1.0 / (self.r * self.r + self.i * self.i  + self.j * self.j + self.k * self.k);

        *self = Quat::new(self.r * a, -self.i * a, -self.j * a, -self.k * a);
        self
    }
}

impl std::fmt::LowerExp for Quat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = format!("{:+.3e}r {:+.3e}i {:+.3e}j {:+.3e}k", self.r, self.i, self.j, self.k);
        f.write_str(&a.as_str())
    }
}

impl Add<Quat> for Quat{
    type Output = Quat;

    fn add(self, rhs: Quat) -> Self::Output {
        Quat{
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl Sub<Quat> for Quat{
    type Output = Quat;

    fn sub(self, rhs: Quat) -> Self::Output {
        Quat{
            r: self.r - rhs.r,
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl Mul<Quat> for Quat{
    type Output = Quat;

    /*
    * (a + bi + cj + ek)(x + yi + zj + wk)
    * a  x + a yi + a zj + a wk => ax  + ayi + azj + awk
    * bi x + biyi + bizj + biwk    bxi - by  + bzk - bwj
    * cj x + cjyi + cjzj + cjwk    cxj - cyk - cz  + cwi
    * ek x + ekyi + ekzj + ekwk    exk + eyj - ezi - ew
    *
    * 1*(ax - by - cz - ew)
    * i*(ay + bx + cw - ez)
    * j*(az - bw + cx + ey)
    * k*(aw + bz - cy + ex)
    */

    fn mul(self, rhs: Quat) -> Self::Output {
        //      |------------|   |------------|   |------------|   |-------------| 
        let r = self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k;
        let i = self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j;
        let j = self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i;
        let k = self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r;
        Quat{ r, i, j, k, }
    }
}

// I am not going to bother putting this behind a macro lmao
impl Mul<f64> for Quat{
    type Output = Quat;

    fn mul(self, rhs: f64) -> Self::Output {
        Quat{
            r: self.r * rhs,
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs,
        }
    }
}

impl Mul<Quat> for f64{
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Self::Output {
        Quat{
            r: rhs.r * self,
            i: rhs.i * self,
            j: rhs.j * self,
            k: rhs.k * self,
        }
    }
}

impl Div<Quat> for Quat{
    type Output = Quat;

    fn div(self, mut rhs: Quat) -> Self::Output {
        self * *rhs.inv()
    }
}

impl Div<f64> for Quat{
    type Output = Quat;

    fn div(self, rhs: f64) -> Self::Output {
        Quat{
            r: self.r / rhs,
            i: self.i / rhs,
            j: self.j / rhs,
            k: self.k / rhs,
        }
    }
}

impl Div<Quat> for f64{
    type Output = Quat;

    fn div(self, mut rhs: Quat) -> Self::Output {
        self * *rhs.inv()
    }
}

