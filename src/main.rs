mod math;

fn main() {
    let v = math::Vector {
        x: 3.0,
        y: 4.0
    };

    let v_length = v.length();
    println!("{0}", v_length);

    let v2 = &v * 2.0;

    let v2_length = v2.length();
    println!("{0}", v2_length);

    let v3 = &v / 2.0;

    let v3_length = v3.length();
    println!("{0}", v3_length);
}
