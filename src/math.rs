use std::ops::{Mul, Div, Add, Sub};

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

    pub fn dot_product<'a>(&self, other: &'a Vector) -> f32
    {
        (self.x * other.x) + (self.y * other.y)
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
    pub fn add_vector<'a>(&self, v: &'a Vector) -> Point
    {
        Point
        {
            x: self.x + v.x,
            y: self.y + v.y
        }
    }
}
