use std::time::{Duration, Instant};
use std::f64::consts::PI;


fn approx_curve_length<F, G, H>(
    a: f64, b: f64, iter: u32, curve: F, incrementor: G, accumulator: H
) where
    F: Fn(f64) -> f64, G: Fn(f64, f64, f64, f64) -> f64, H: Fn(f64, f64) -> f64 {    
    /* Calculate the curve length approximation of an continuous function over range [a, b] */
    println!("Running with {:?} iterations.", iter);
    let start = Instant::now();
    let mut buf : [f64; 2] = [0.0, curve(a)];
    let mut count: f64 = 0.0;
    let delta = (b-a) / (iter as f64);
    for i in 1..iter+1 {
        buf[0] = buf[1];
        buf[1] = curve(a + delta * i as f64);
        let theta_avg = a + delta * (i as f64 - 0.5);
        count = count + incrementor(buf[0], buf[1], delta, theta_avg);
    }
    let duration = start.elapsed();
    println!("Got result in: {:?}:", duration);
    println!("Curve Length Approximation: {:?}\n", accumulator(count, delta) * 2.0);
}

fn trials<F, G, H>(a: f64, b: f64, f: &F, inc: &G, acc: &H) where
    F: Fn(f64) -> f64, G: Fn(f64, f64, f64, f64) -> f64, H: Fn(f64, f64) -> f64 {
    for iter in [100, 1000, 10000, 100000, 1000000, 10000000].iter() {
        approx_curve_length(a, b, *iter, f, inc, acc);
    }
}


fn main() {
    /* The ellipse is horizontal, centered at (0, 0), and with a semi-major length == 3 and
     * semi minor length == 1. 
     * We just care about the upper half of the ellipse in this function
     */
    let upper_ellipse_cart = |x: f64| {
        (1.0 - (x*x) / 9.0).sqrt()
    };

    let upper_circle_cart = |x: f64| {
        (1.0 - x*x).sqrt()
    };

    let upper_circle_polar = |theta: f64| {
        1.0 / (theta.sin().powi(2) + theta.cos().powi(2)).sqrt()
    };

    /* polar helpers */
    let upper_ellipse_polar_squared = |theta: f64| {
        9.0 / (9.0 * theta.sin() * theta.sin() + theta.cos() * theta.cos())
    };

    let upper_ellipse_polar = |theta: f64| {
        upper_ellipse_polar_squared(theta).sqrt()
    };

    let upper_ellipse_polar_deri = |theta: f64| {
        - 24.0 * theta.sin() * theta.cos() / (8.0 * theta.sin().powi(2) + 1.0).powi(3).sqrt()
    };

    /* polygon approx */
    let poly_incre = |l: f64, r:f64, d: f64, _: f64| {
        ((r - l) * (r - l) + d * d).sqrt()
    };
    let poly_accu = |x, _| { x };

    /* grid-based approx */
    let grid_incre = |l: f64, r:f64, d: f64, _: f64| {
        ((r - l).abs() / d).ceil()
    };
    let grid_accu = |x: f64, d: f64| -> f64 { x * d };

    /* polar differential approx */
    let polar_incre = |_l: f64, _r:f64, d: f64, th: f64| {
        d * (upper_ellipse_polar_squared(th) + upper_ellipse_polar_deri(th).powi(2)).sqrt()
    };
    let polar_accu = |x, _| { x };

    /* tried and failed polar approxes */
    // the following methods will not work on complex curves.
    // I forgot how to calculus for a sec there.
    let polar_incre_mid = |l: f64, r:f64, d: f64, _: f64| {
        (r+l) * d
    };
    // let polar_incre_linear_iterpol = |l: f64, r:f64, d: f64| {
    //     ((r - l).powi(2) + d * d).sqrt()
    // };

    println!("========================= Polygon-Ellipse =========================");
    trials(-3.0, 3.0, &upper_ellipse_cart, &poly_incre, &poly_accu);
    // println!("========================= Grid-Ellipse =========================");
    // trials(-3.0, 3.0, &upper_ellipse_cart, &grid_incre, &grid_accu);
    println!("========================= Polygon-Circle =========================");
    trials(-1.0, 1.0, &upper_circle_cart, &poly_incre, &poly_accu);
    // println!("========================= Grid-Circle =========================");
    // trials(-1.0, 1.0, &upper_circle_cart, &grid_incre, &grid_accu);
    println!("========================= Polar-Ellipse (with polar integral) =========================");
    trials(0.0, PI, &upper_ellipse_polar, &polar_incre, &polar_accu);
    println!("========================= Polar-Ellipse (Epic Fail) =========================");
    trials(0.0, PI, &upper_ellipse_polar, &polar_incre_mid, &polar_accu);
    println!("========================= Polar-Circle =========================");
    trials(0.0, PI, &upper_circle_polar, &polar_incre_mid, &polar_accu);
}
