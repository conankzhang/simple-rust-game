use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Matrix
{
    pub x: Vector,
    pub y: Vector,
    pub z: Vector,
    pub w: Vector,
}
