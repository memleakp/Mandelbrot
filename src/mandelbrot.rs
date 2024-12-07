use crate::complex::Complex;
use crate::image::*;

pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
    let mut z: Complex = Complex::new(0.0, 0.0);

    for iteration in 0..max_iterations {
        z = z * z + c;
        
        if z.mag() > 4.0 {
            // The pixel (x, y) is not in the Mandelbrot set
            return Option::Some(iteration);
        }
    }

    None
}

pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
    // your code here
    Image::new(width, height)
}

