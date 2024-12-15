mod math;

fn main() {
    let p1 = math::Point {
        x: 3.0,
        y: 4.0
    };

    let p2 = math::Point {
        x: 1.0,
        y: 2.0
    };

    let v2 = &p1 - &p2;
    let normalized = v2.normalized();

    println!("{0},{1}", normalized.x, normalized.y);
    println!("{0}", normalized.length());
}
