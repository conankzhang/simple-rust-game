pub mod math;

fn main() {
    let p1 = math::Point {
        x: 0.0,
        y: -1.0
    };

    let p2 = math::Point{
        x: 1.0,
        y: 1.0
    };

    let v = p1 - p2;
    let length = v.length();

    println!("Result: {0}", length);
}
