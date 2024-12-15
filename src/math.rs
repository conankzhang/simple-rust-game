use std::ops::Sub;

pub struct Vector
{
    pub x: f32,
    pub y: f32
}

impl Vector
{
    pub fn length(&self) -> f32
    {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn length_squared(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y)
    }
}

pub struct Point
{
    pub x: f32,
    pub y: f32
}

impl<'a> Sub for &'a Point
{
    type Output = Vector;
    fn sub(self, other: &'a Point) -> Vector
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
