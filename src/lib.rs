use image::{ImageBuffer, ImageResult, RgbaImage};
use num::Complex;
use std::cell::RefCell;

type Point = (u32, u32);

#[derive(Clone)]
pub enum RenderType {
    Julia,
    Mandelbrot,
    Buddhabrot,
}

pub struct Render {
    buffer: RefCell<RgbaImage>,
    upper_left: Complex<f64>,
    ratio: f64,
    filename: String,
    render_type: RenderType,
    c: Complex<f64>,

    #[allow(dead_code)]
    bail_out: u32,
}

impl Render {
    fn julia(&self, z: Complex<f64>, c: Complex<f64>) -> Option<u8> {
        let mut z = z;
        for i in 0..self.bail_out {
            z = z * z + c;
            if z.norm() > 4.0 {
                return Some(i as u8);
            }
        }
        None
    }
    fn mandelbrot(&self, c: Complex<f64>) -> Option<u8> {
        let z = Complex { re: 0.0, im: 0.0 };

        self.julia(z, c)
    }

    #[allow(dead_code)]
    fn buddhabrot(&self, c: Complex<f64>) -> Option<Vec<Complex<f64>>> {
        let mut weg = Vec::new();
        let mut z = Complex { re: 0.0, im: 0.0 };

        for _ in 0..self.bail_out {
            z = z * z + c;
            weg.push(z);
            if z.norm() > 4.0 {
                return Some(weg);
            }
        }
        None
    }

    // transform pixel coordenads to
    // complex coord
    fn pixel_to_complex(&self, pixel: Point) -> Complex<f64> {
        Complex {
            re: self.upper_left.re + pixel.0 as f64 * self.ratio,
            im: self.upper_left.im - pixel.1 as f64 * self.ratio,
        }
    }

    #[allow(dead_code)]
    fn complex_to_pixel(&self, complex: Complex<f64>) -> Point {
        let x = complex.re - self.upper_left.re / self.ratio;
        let y = -complex.im + self.upper_left.re / self.ratio;
        (x as u32, y as u32)
    }

    pub fn run(&self) -> ImageResult<()> {
        let mut b = self.buffer.borrow_mut();
        for (row, colum, pixel) in b.enumerate_pixels_mut() {
            let complex_coord = self.pixel_to_complex((row, colum));
            let r = match self.render_type {
                RenderType::Julia => match self.julia(complex_coord, self.c) {
                    Some(v) => 255 - v,
                    None => 0,
                },
                RenderType::Mandelbrot => match self.mandelbrot(complex_coord) {
                    Some(v) => 255 - v,
                    None => 0,
                },
                _ => 0,
            };
            *pixel = image::Rgba([
                r,
                (0.1 * row as f32) as u8,   // gradient
                (0.1 * colum as f32) as u8, // gradient
                255,
            ]);
        }
        b.save(self.filename.as_str())?;
        Ok(())
    }
}

// This dont make much sense now
// but, in the future the render could be made using
// the paar center, ratio or the upper_left, lower
#[derive(Clone)]
pub struct RenderBuilder {
    buffer: Option<RgbaImage>,
    center: Complex<f64>,
    ratio: f64,
    filename: String,
    render_type: RenderType,
    bail_out: u32,
    c: Complex<f64>,
}

impl RenderBuilder {
    pub fn new() -> Self {
        Self {
            buffer: None,
            center: Complex { re: 0.0, im: 0.0 },
            ratio: 0.001,
            filename: "julia.png".to_string(),
            render_type: RenderType::Julia,
            bail_out: 255,
            c: Complex { re: 0.0, im: 0.0 },
        }
    }
    pub fn size_image(self, width: u32, height: u32) -> Self {
        let buffer = ImageBuffer::new(width, height);
        Self {
            buffer: Some(buffer),
            ..self
        }
    }
    pub fn center(self, re: f64, im: f64) -> Self {
        let center = Complex { re, im };
        Self { center, ..self }
    }
    pub fn set_ratio(self, ratio: f64) -> Self {
        Self { ratio, ..self }
    }
    pub fn set_type(self, render_type: RenderType) -> Self {
        Self {
            render_type,
            ..self
        }
    }
    pub fn set_c(self, re: f64, im: f64) -> Self {
        let c = Complex { re, im };
        Self { c, ..self }
    }

    pub fn set_bailout(self, bail_out: u32) -> Self {
        Self { bail_out, ..self }
    }
    pub fn set_filename(self, filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            ..self
        }
    }

    pub fn build(self) -> Result<Render, &'static str> {
        if let Some(buffer) = self.buffer {
            let ratio = self.ratio;
            let center = self.center;
            let filename = self.filename;
            let render_type = self.render_type;
            let bail_out = self.bail_out;
            let c = self.c;

            let upper_left = Complex {
                re: center.re - buffer.width() as f64 / 2.0 * ratio,
                im: center.im + buffer.height() as f64 / 2.0 * ratio,
            };

            let buffer = RefCell::new(buffer);
            Ok(Render {
                buffer,
                upper_left,
                ratio,
                filename,
                render_type,
                bail_out,
                c,
            })
        } else {
            Err("need to specifie size of image")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_renderbuilder() {
        let upper_left = Complex { re: -1.0, im: 0.75 };
        let lower_right = Complex { re: 1.0, im: -0.75 };

        let width_complex = lower_right.re - upper_left.re;

        let bounds = (1000, 750);
        let ratio = width_complex / bounds.0 as f64;

        let b = RenderBuilder::new()
            .size_image(1000, 750)
            .center(0.0, 0.0)
            .set_ratio(ratio)
            .build()
            .unwrap();

        assert_eq!(b.upper_left, upper_left)
    }
}
