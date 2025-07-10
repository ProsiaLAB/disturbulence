use crate::modes::BCType;
use crate::types::RMatrix;
use crate::types::RVector;

pub struct ThermalSetup {
    pub gamma: f64,
    pub isothermal: bool,
    pub apply_newtonian_cooling: bool,
    pub tau_newtonian_cooling: f64,
    pub ci_squared_initial: RMatrix,
}

pub struct BCSetup {
    pub rmin: BCType,
    pub rmax: BCType,
    pub zmin: BCType,
    pub zmax: BCType,
    pub balanced: bool,
    pub d_ci_dr_inner: RVector,
    pub d_ci_dr_outer: RVector,
    pub c_sound_rmin: f64,
    pub c_sound_rmax: f64,
    pub isothermal: bool,
}

pub fn set_up_thermal_parameters(setup: ThermalSetup) {
    todo!()
}

pub fn set_up_boundary_conditions(setup: BCSetup) {
    todo!()
}
