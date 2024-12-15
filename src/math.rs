pub struct Vector
{
    pub x: f32,
    pub y: f32
}

pub struct Point
{
    pub x: f32,
    pub y: f32
}

impl Point
{
    pub fn add_vector(&self, v: Vector) -> Point
    {
        Point
        {
            x: self.x + v.x,
            y: self.y + v.y
        }
    }
}
