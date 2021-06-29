use juliaset::RenderBuilder;
use juliaset::RenderType::*;

fn main() {
    let r = RenderBuilder::new().size_image(4000, 3000);

    let r_julia = r
        .clone()
        .set_c(-0.8, 0.156)
        .set_filename("julia.png")
        .set_type(Julia)
        .build()
        .unwrap();

    r_julia.run().unwrap();

    let r_mandel = r
        .set_filename("mandelbrot.png")
        .set_type(Mandelbrot)
        .build()
        .unwrap();

    r_mandel.run().unwrap();
}
