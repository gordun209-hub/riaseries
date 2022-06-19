use num::complex::Complex;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
// use std::time::{Duration, Instant};
fn main() {
    let a = 10;
    let b = Box::new(20); // <--- Integer on heap also known as boxed integer
    let c = Rc::new(Box::new(30)); // <--- Boxed integer wrapped within a reference counter
    let d = Arc::new(Mutex::new(40)); // <-- Integer wrapped in an atomic tef counter and
                                      // protected by a mutual exclusion lock
    let ab = Complex { re: 2.1, im: -1.2 };
    let bb = Complex::new(11.1, 22.2);
    let res = ab + bb;

    println!("{} + {}i", res.re, res.im);
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?} ", a, b, c, d);

    // let mut count = 0;
    // let time_limit = Duration::new(1, 0);
    // let start = Instant::now();
    //
    // while (Instant::now() - start) < time_limit {
    //     count += 1;
    // }
    // println!("{}", count);
    let needle = 1;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        let res = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
        if res == "hit!" {
            println!("{}: {}", item, res);
        }
        if *item == needle {
            println!("{} ", item);
        }
    }
    let mandelbrot = calculate_mandelbrot(1000,-2.0,1.0,-1.0,1.0,100,12);
    render_mandelbrot(mandelbrot);
}

fn calculate_mandelbrot(
    // <2>
    max_iters: usize, // <3>
    x_min: f64,       // <4>
    x_max: f64,       // <4>
    y_min: f64,       // <4>
    y_max: f64,       // <4>
    width: usize,     // <5>
    height: usize,    // <5>
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width); // <6>
    for img_y in 0..height {
        // <7>

        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent; // <8>
            let cy = y_min + (y_max - y_min) * y_percent; // <8>
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(
    // <9>
    cx: f64,
    cy: f64,
    max_iters: usize,
) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 }; // <10>
    let c = Complex::new(cx, cy); // <11>

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            // <12>
            return i;
        }
        z = z * z + c; // <13>
    }
    max_iters // <14>
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}
