extern crate image;
extern crate num;

use image::{ImageBuffer, RgbaImage};
use num::Complex;

fn julia(z: Complex<f64>, c: Complex<f64>) -> u8 {
    let mut z = z;
    let mut i = 0;
    while i < 255 && z.norm() < 4.0 {
        z = z * z + c;
        i += 1;
    }

    i
}

fn mandelbrot(c: Complex<f64>) -> u8 {
    let z = Complex { re: 0.0, im: 0.0 };

    julia(z, c)
}

fn pixel_to_point(
    bounds: (u32, u32),
    pixel: (u32, u32),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render_mandelbrot(buffer: &mut RgbaImage, upper_left: Complex<f64>, lower_right: Complex<f64>) {
    let b = (buffer.width(), buffer.height());
    for (row, colum, pixel) in buffer.enumerate_pixels_mut() {
        let point = pixel_to_point(b, (colum, row), upper_left, lower_right);
        *pixel = image::Rgba([
            255 - mandelbrot(point),    // mandelbrot
            (0.1 * row as f32) as u8,   // gradient
            (0.1 * colum as f32) as u8, // gradient
            255,
        ]);
    }
}

fn render(
    buffer: &mut RgbaImage,
    c: Complex<f64>,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let b = (buffer.width(), buffer.height());
    for (row, colum, pixel) in buffer.enumerate_pixels_mut() {
        let point = pixel_to_point(b, (colum, row), upper_left, lower_right);
        *pixel = image::Rgba([
            255 - julia(point, c),      // julia value
            (0.1 * row as f32) as u8,   // gradient
            (0.1 * colum as f32) as u8, // gradient
            255,
        ]);
    }
}

fn main() {
    let bounds = (4000, 3000);
    let upper_left = Complex { re: -1.2, im: 0.50 };
    let lower_right = Complex { re: 1.2, im: -0.50 };

    let c = Complex {
        re: -0.8,
        im: 0.156,
    };

    let mut buffer = ImageBuffer::new(bounds.0, bounds.1);
    render(&mut buffer, c, upper_left, lower_right);

    buffer.save("julia.png").unwrap();

    let mut buffer = ImageBuffer::new(bounds.0, bounds.1);
    render_mandelbrot(&mut buffer, upper_left, lower_right);
    buffer.save("mandelbrot.png").unwrap();
}
