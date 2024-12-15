use std::ops::Sub;

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

impl Sub for Point
{
    type Output = Vector;
    fn sub(self, other: Point) -> Vector
    {
        Vector
        {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
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
