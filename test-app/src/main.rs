use basic_math::stats;
use basic_math::linear;

fn main() {
    let v = vec![1.0, 2.0, 3.0];

    println!("mean = {}", stats::mean(&v));
    println!("dot  = {}", linear::dot(&v, &v));


}
