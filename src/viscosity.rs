use crate::core::ViscosityType;

#[derive(Default)]
pub struct Viscosity {
    pub apply_viscosity: bool,
    pub viscosity_type: ViscosityType,
}
