pub mod math;

fn main() {
    let p = math::Point {
        x: 1.0,
        y: 0.0
    };

    let v = math::Vector{
        x: 2.0,
        y: 3.0
    };

    let r = p.add_vector(v);

    println!("Result: {0}, {1}", r.x, r.y);
}
