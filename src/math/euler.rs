use super::vector::Vector3;

#[derive(Clone, Debug)]
pub struct Euler
{
    pub pitch : f32,
    pub yaw : f32,
    pub roll : f32,
}

impl Euler {
    pub fn to_vector(&self) -> Vector3
    {
        let pitch_cos = self.pitch.cos();
        Vector3 {
            x: self.yaw.cos() * pitch_cos,
            y: self.yaw.sin() * pitch_cos,
            z: self.pitch.sin(),
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
