use std::time::Duration;

use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use binds::image::*;

fn convolute(img: &Vec<Vec<f64>>, k : &[[f64; 3]; 3]) -> Vec<Vec<f64>>{
    let mut n = img.clone();

    //let height = img.len();
    //let width  = img[0].len();
    let height = img[0].len();
    let width  = img   .len();
    for indx in 0..(width * height){
        let i = indx % width;
        let j = indx / width;

        let mut i_min : i32 = -1;
        let mut i_max : i32 =  2;
        let mut j_min : i32 = -1;
        let mut j_max : i32 =  2;

        if i < 1 { i_min = 0; }
        if j < 1 { j_min = 0; }

        if i + 2 > width { i_max = 1; }
        if j + 2 > height{ j_max = 1; }

        let mut result = 0.0;
        for si in i_min..i_max{
            for sj in j_min..j_max{
                let ni = (i as i32 + si) as usize;
                let nj = (j as i32 + sj) as usize;
                let si = (1 + si) as usize;
                let sj = (1 + sj) as usize;
                
                result += img[ni][nj] * k[si][sj];
            }
        }

        n[i][j] = (result / 2.0).abs();
    }

    return n;
}

// fucking awful way to do it
fn make_vector<T: Default + Clone>(width: usize, height: usize)-> Vec<Vec<T>>{
    let mut org = Vec::<T>::new();
    org.resize(height, T::default());
    let mut r = Vec::<Vec::<T>>::new();
      r.resize(width, org);


    return r; 
}

fn main(){
    let img = Img::load("test.png").unwrap();
    let hkernel = [
        [ 0.50,  0.00, -0.50,],
        [ 1.00,  0.00, -1.00,],
        [ 0.50,  0.00, -0.50,],
    ];
    let vkernel = [
        [ 0.50,  1.00,  0.50,],
        [ 0.00,  0.00,  0.00,],
        [-0.50, -1.00, -0.50,],
    ];

    let mut map = make_vector::<f64>(img.width, img.height);

    for indx in 0..(img.width * img.height){
        let i = indx % img.width;
        let j = indx / img.width;
        
        map[i][j] = img.get_pixel(i, j)[0].clone() as f64 / 255.0;
    }

    let vmap = convolute(&map, &vkernel);
    let hmap = convolute(&map, &hkernel);

    let mut nimg = make_vector::<u8>(img.width, img.height);

    for indx in 0..(img.width * img.height){
        let i = indx % img.width;
        let j = indx / img.width;

        let sum = vmap[i][j] + hmap[i][j];
        nimg[i][j] = (sum * 127.5) as u8;
    }

    let nwimg = Img::from_vec1(nimg).unwrap();
    println!("writing");
    match nwimg.write(ImgFmt::PNG, "test_r.png") {
        Err(_) => println!("fuck"),
        _ => {}
    }
}

#[allow(dead_code)]
fn fake_main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let window = video_subsystem.window("test", 800, 600)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    let color = Color::RGB(0, 0, 0);
    'main_loop: loop{
        canvas.set_draw_color(color);
        canvas.clear();

        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                },
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }

}
