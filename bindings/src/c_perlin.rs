extern "C" {
    pub fn perlin_noise_seed(
        x: f32,
        y: f32,
        z: f32,
        xwrap: u64,
        ywrap: u64,
        zwrap: u64,
        seed: u64,
    ) -> f32;
}
