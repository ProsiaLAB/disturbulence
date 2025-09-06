use extensions::types::{RMatrix, RVector};

#[derive(Default)]
pub struct Gravity {
    pub graviy_flag: bool,
    pub gravity_type: GravityType,
    pub gm_constant: f64,
    pub gz_uniform: f64,
    pub gr: RMatrix,
    pub gz: RMatrix,
    pub u_kepler: RVector,
    pub omega_kepler: RVector,
}

#[derive(Default)]
pub enum GravityType {
    #[default]
    MassAtOrigin,
    ThinDiskMassAtOrigin,
    ThinDiskMassAtOriginNoRadial,
    UniformGz,
}
