use math::algebra::base_matrix::BaseMat;

fn main(){
    let a = BaseMat::<f32, 2, 2>{data: [
        [1., 2.,],
        [3., 4.,],
    ]};

    println!("{a} {}", a.data[1][0])
}
