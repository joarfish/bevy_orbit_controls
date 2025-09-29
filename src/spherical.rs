use bevy::math::Vec3;
use bevy::prelude::Reflect;
use std::f32::consts::PI;

const MARGIN: f32 = 0.000001;

#[derive(Reflect, PartialOrd, PartialEq, Clone, Copy)]
pub struct SphericalCoords {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
}

impl SphericalCoords {
    pub fn clamp_phi(&mut self) {
        self.phi = self.phi.clamp(MARGIN, PI - MARGIN);
    }
}

impl From<Vec3> for SphericalCoords {
    fn from(value: Vec3) -> Self {
        let mut coords = SphericalCoords {
            radius: 1.0,
            theta: 0.0,
            phi: 0.0,
        };

        coords.radius = value.length();

        if coords.radius == 0.0 {
            coords.theta = 0.0;
            coords.phi = 0.0;
        } else {
            coords.theta = value.x.atan2(value.z);
            coords.phi = f32::acos((value.y / coords.radius).clamp(-1.0, 1.0));
        }
        
        coords
    }
}

impl From<SphericalCoords> for Vec3 {
    fn from(value: SphericalCoords) -> Self {
        Vec3 {
            x: value.radius * f32::sin(value.phi) * f32::sin(value.theta),
            y: value.radius * f32::cos(value.phi),
            z: value.radius * f32::sin(value.phi) * f32::cos(value.theta),
        }
    }
}