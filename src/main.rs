mod math;
use std::time::Instant;

struct Character
{
    position : math::Vector,
    velocity: math::Vector,
    gravity: math::Vector
}

fn main() {
    let mut character = Character
    {
        position: math::Vector{x: 0.0, y: 0.0},
        velocity: math::Vector{x: 2.0, y: 2.0},
        gravity: math::Vector{x: 0.0, y: -2.0},
    };

    let mut current_time = Instant::now();

    let mut run_loop = true;

    while run_loop
    {
        let previous_time = current_time;
        current_time = Instant::now();

        let delta_time = current_time.duration_since(previous_time);

        update(delta_time.as_secs_f32(), &mut character);
        draw();

        if false
        {
            run_loop = false;
        }
    }
}

fn update(delta_time : f32, character : & mut Character)
{
    character.position = &character.position + &(&character.velocity * delta_time);
    character.velocity = &character.velocity + &(&character.gravity * delta_time);

    println!("{0},{1}", character.position.x, character.position.y);
}

fn draw()
{

}
