pub mod math;

fn main() {
    let p = math::Point {
        x: 0.0,
        y: -1.0
    };

    let i = math::Point{
        x: 1.0,
        y: 1.0
    };

    let c = math::Point{
        x: 2.0,
        y: -1.0
    };

    let ip = &p - &i;
    let cp = &p - &c;
    let length_squared_ip = ip.length_squared();
    let length_squared_cp = cp.length_squared();

    println!("IP: {0}", length_squared_ip);
    println!("CP: {0}", length_squared_cp);
}
