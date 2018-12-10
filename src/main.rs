//! # monte_carlo
//! 
//! Estimate the value of pi by calculating the poroportion of random points
//! inside/outisde of a circle

extern crate rand;
use rand::prelude::*;

extern crate rayon;
use rayon::prelude::*;

// Random float 0.0 - 1.0
fn r_gen() -> f64 {
    rand::thread_rng().gen()
}

// Distance between 0,0 and a point in 2D space
fn distance(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn main() {
    // The number of iterations
    const COUNT: usize = 10_000_000_000;

    // Sum the number of points closer than 1 unit from 0,0
    let sum: usize = (0 .. COUNT).into_par_iter().map(|_|
        if distance(r_gen(), r_gen()) < 1.0 {
            1
        } else {
            0
        }
    ).sum();

    // Calculate the proportion of inside/outside points
    // Mulitply by 4 to account for the other quadrants of the circle
    let average = sum as f64 / COUNT as f64 * 4.0;

    println!("{}", average);

}
