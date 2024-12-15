mod math;

fn main() {
    let p1 = math::Vector {
        x: 4.0,
        y: 0.0
    };

    let p2 = math::Vector {
        x: 0.0,
        y: -5.0
    };

    let v2 = &p1 + &p2;

    println!("{0},{1}", v2.x, v2.y);
}
