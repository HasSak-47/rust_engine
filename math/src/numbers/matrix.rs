use std::ops::{Add, Mul, Sub, Div};
use super::quaternion::Quat;

#[derive(Clone, Copy, Debug)]
struct Mat4{
    pub d : [[f32;4];4],
}

impl Default for Mat4{
    fn default() -> Self {
        IDENTITY.clone()
    }
}

const IDENTITY : Mat4 = Mat4{
    d:[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
};

const ZEROS : Mat4 = Mat4{
    d:[
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
};

impl Add<Mat4> for Mat4{
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Self::Output {
        let mut output = ZEROS.clone();

        for o in 0..16{
            let i = o % 4;
            let j = o / 4;

            output.d[i][j] = self.d[i][j] + rhs.d[i][j];
        }

        output
    }
}
