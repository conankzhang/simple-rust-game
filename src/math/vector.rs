use std::ops::{Add, Div, DivAssign, Mul, Sub};


pub fn dot_product(lhs: Vector, rhs: Vector) -> f32
{
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z) + (lhs.w * rhs.w)
}

pub fn cross_product(lhs: Vector, rhs: Vector) -> Vector
{
    Vector
    {
        x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
        y: (lhs.z * rhs.x) + (lhs.x * rhs.z),
        z: (lhs.x * rhs.y) + (lhs.y * rhs.x),
        w: 0.0
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

    pub fn normalized(self) -> Vector
    {
        self / self.length()
    }

    pub fn normalize(&mut self)
    {
        *self /= self.length()
    }
}

impl Add for Vector
{
    type Output = Vector;
    fn add(self, other: Vector) -> Vector
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

impl Sub for Vector
{
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector
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

impl Mul<f32> for Vector
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

impl Div<f32> for Vector
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

impl DivAssign<f32> for Vector
{
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}
