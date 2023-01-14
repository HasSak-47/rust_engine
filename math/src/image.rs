use image::{ImageBuffer, GenericImageView, Pixel, Primitive};

pub fn filter3x3<I, P, S>(image: &I, kernel: &[f32], clamper: fn(f32,f32,f32,f32) -> (S,S,S,S)) -> ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    // The kernel's input positions relative to the current pixel.
    let taps: &[(isize, isize)] = &[
        (-1, -1), ( 0, -1), ( 1, -1),
        (-1,  0), ( 0,  0), ( 1,  0),
        (-1,  1), ( 0,  1), ( 1,  1),
    ];

    let (width, height) = image.dimensions();

    let mut out = ImageBuffer::new(width, height);

    let sum = match kernel.iter().fold(0.0, |s, &item| s + item) {
        x if x == 0.0 => 1.0,
        sum => sum,
    };
    let sum = (sum, sum, sum, sum);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut t = (0.0, 0.0, 0.0, 0.0);

            // TODO: There is no need to recalculate the kernel for each pixel.
            // Only a subtract and addition is needed for pixels after the first
            // in each row.
            for (&k, &(a, b)) in kernel.iter().zip(taps.iter()) {
                let k = (k, k, k, k);
                let x0 = x as isize + a;
                let y0 = y as isize + b;

                let p = image.get_pixel(x0 as u32, y0 as u32);

                #[allow(deprecated)]
                let (k1, k2, k3, k4) = p.channels4();


                let vec: (f32, f32, f32, f32) = (
                    k1.to_f32().unwrap(),
                    k2.to_f32().unwrap(),
                    k3.to_f32().unwrap(),
                    k4.to_f32().unwrap(),
                );

                t.0 += vec.0 * k.0;
                t.1 += vec.1 * k.1;
                t.2 += vec.2 * k.2;
                t.3 += vec.3 * k.3;
            }

            let (t1, t2, t3, t4) = clamper(t.0 / sum.0, t.1 / sum.1, t.2 / sum.2, t.3 / sum.3);

            #[allow(deprecated)]
            let t = Pixel::from_channels(t1, t2, t3, t4);

            out.put_pixel(x, y, t);
        }
    }

    out
}
