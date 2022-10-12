
extern "C" {
    pub fn stb_perlin_noise3(x: f32, y: f32, z: f32, x_wrap: i32, y_wrap: i32, z_wrap: i32) -> f32;
    pub fn stb_perlin_noise3_seed(x: f32, y: f32, z: f32, x_wrap: i32, y_wrap: i32, z_wrap: i32, seed: i32) -> f32;
}
