mod lily;
use lily::{
    math::transform2d::*,
    math::generator::wave::*,
    image::*
};

#[derive(Copy, Clone, Eq, PartialEq, Default)]
enum Tborder {
    Pass = 0b010,

    #[default]
    Empty = 0b000,
}

impl Opposite for Tborder{
    fn opposite(&self) -> Self{
        *self
    }
}

fn main() {
    let mut values = vec![
        [
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
        ],
        [
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8, 255u8, 255u8, 255u8,   0u8,],
            [255u8, 255u8, 255u8, 255u8, 255u8,],
            [  0u8, 255u8, 255u8, 255u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
        ],
        [
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
        ],
        [
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8, 255u8,   0u8,   0u8,],
            [255u8, 255u8, 255u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
            [  0u8,   0u8,   0u8,   0u8,   0u8,],
        ],
    ];

    values.push(values[2].rotate(1));
    values.push(values[3].rotate(1));
    values.push(values[3].rotate(2));
    values.push(values[3].rotate(3));


    let mut units = vec![
        Unit::<Tborder>::new(Tborder::Empty, Tborder::Empty, Tborder::Empty, Tborder::Empty),
        Unit::<Tborder>::new(Tborder::Pass, Tborder::Pass, Tborder::Pass, Tborder::Pass),
        Unit::<Tborder>::new(Tborder::Pass, Tborder::Pass, Tborder::Empty, Tborder::Empty),
        Unit::<Tborder>::new(Tborder::Pass, Tborder::Empty, Tborder::Empty, Tborder::Pass),
    ];
    units.push(units[2].rotate(1));
    units.push(units[3].rotate(3));
    units.push(units[3].rotate(2));
    units.push(units[3].rotate(1));

    for unit in &units{
        println!("{} {} {} {}",
            unit.north == Tborder::Pass,
            unit.south == Tborder::Pass,
            unit.east  == Tborder::Pass,
            unit.west  == Tborder::Pass,
            );
    }

    let size = 10;
    let mut board = FiniteMap::new(size, size, units, 1);
    board.determine();


    let mut image = Img::new(5 * size, 5 * size, 1).unwrap();
    for __j in 0..size{
        let j = (size - 1) - __j;
        for i in 0..size{
            let index = board.map[i][j].collapse_val();
            //let index = if j == 0 && i < board.possible.len(){
            //    i
            //}
            //else{
            //    0
            //};
            print!("{} ", index);
            for i1 in 0..5{
                for j1 in 0..5{
                    *image.get_channel(5 * i + i1, 5 * __j + j1, 0) = values[index][j1][i1];
                }
            }
        }
        println!();
    }

    image.write(ImgFmt::PNG, "test.png").unwrap();
}
