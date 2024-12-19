use std::ops::{Add, Div, DivAssign, Mul, Sub};

pub fn dot_product(lhs: Vector3, rhs: Vector3) -> f32
{
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
}

pub fn cross_product(lhs: Vector3, rhs: Vector3) -> Vector3
{
    Vector3
    {
        x: (lhs.y * rhs.z) - (lhs.z * rhs.y),
        y: (lhs.z * rhs.x) + (lhs.x * rhs.z),
        z: (lhs.x * rhs.y) + (lhs.y * rhs.x),
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Vector2
{
    pub x: f32,
    pub y: f32,
}

impl Vector2
{
    pub const fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vector3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector3
{
    pub const fn new(x: f32, y: f32, z:f32) -> Self {
        Self{x, y, z}
    }

    pub fn length(&self) -> f32
    {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn length_squared(&self) -> f32
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn normalized(self) -> Vector3
    {
        self / self.length()
    }

    pub fn normalize(&mut self)
    {
        *self /= self.length()
    }
}

impl Add for Vector3
{
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3
    {
        Vector3
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3
{
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3
    {
        Vector3
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3
{
    type Output = Vector3;
    fn mul(self, scalar: f32)-> Vector3
    {
        Vector3
        {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vector3
{
    type Output = Vector3;
    fn div(self, scalar: f32)-> Vector3
    {
        Vector3
        {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f32> for Vector3
{
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}
