use super::vector::Vector;

#[derive(Clone, Debug)]
pub struct Euler
{
    pub pitch : f32,
    pub yaw : f32,
    pub roll : f32,
}

impl Euler {
    pub fn to_vector(&self) -> Vector
    {
        let pitch_cos = self.pitch.cos();
        Vector {
            x: self.yaw.cos() * pitch_cos,
            y: self.yaw.sin() * pitch_cos,
            z: self.pitch.sin(),
            w:0.0
        }
    }

    pub fn normalize(&mut self)
    {
        if self.pitch > 89.0
        {
            self.pitch = 89.0;
        }

        if self.pitch < -89.0
        {
            self.pitch = -89.0;
        }

        while self.yaw < -180.0
        {
           self.yaw += 360.0;
        }

        while self.yaw > 180.0
        {
           self.yaw -= 360.0;
        }
    }
}
