pub mod vector;
pub mod matrix;
pub mod euler;

pub fn approach(goal: f32, current: f32, delta_time: f32) -> f32
{
    let difference = goal - current;

    if difference > delta_time
    {
        return current + delta_time;
    }

    if difference < -delta_time
    {
        return current - delta_time;
    }
    else {
        return goal;
    }
}
