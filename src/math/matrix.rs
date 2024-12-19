use super::vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Matrix
{
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
    pub w: Vector3,
}
