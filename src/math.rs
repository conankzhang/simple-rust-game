use std::ops::{Add, Sub, Mul, Div};

pub fn dot_product<'a, 'b>(first: &'a Vector, second: &'b Vector) -> f32
{
    (first.x * second.x) + (first.y * second.y) + (first.z * second.z) + (first.w * second.w)
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


#[derive(Copy, Clone, Debug)]
pub struct Vector
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector
{
    const fn new(x: f32, y: f32, z:f32, w:f32) -> Self {
        Self{x, y, z, w}
    }

    pub fn length(&self) -> f32
    {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn length_squared(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
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
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w * other.w,
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
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
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
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
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
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}
