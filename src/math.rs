use std::ops::{Add, Sub, Mul, Div};

pub fn dot_product<'a, 'b>(first: &'a Vector, second: &'b Vector) -> f32
{
    (first.x * second.x) + (first.y * second.y)
}

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

    pub fn normalized(&self) -> Vector
    {
        self / self.length()
    }
}

impl<'a> Add for &'a Vector
{
    type Output = Vector;
    fn add(self, other: &'a Vector) -> Vector
    {
        Vector
        {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a> Sub for &'a Vector
{
    type Output = Vector;
    fn sub(self, other: &'a Vector) -> Vector
    {
        Vector
        {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<'a> Mul<f32> for &'a Vector
{
    type Output = Vector;
    fn mul(self, scalar: f32)-> Vector
    {
        Vector
        {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

impl<'a> Div<f32> for &'a Vector
{
    type Output = Vector;
    fn div(self, scalar: f32)-> Vector
    {
        Vector
        {
            x: self.x / scalar,
            y: self.y / scalar
        }
    }
}
