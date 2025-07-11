use crate::modes::BCType;
use crate::types::RMatrix;
use crate::types::RTensor;
use crate::types::RTensor4;
use crate::types::RVector;

pub struct Control {
    pub istep: usize,
    pub nsteps: usize,
    pub t: f64,
    pub cfl: f64,
    // pub file_state: FileState,
}

pub struct Grid {
    /// Has the grid been set-up?
    pub gridded: bool,
    /// Is the grid periodic?
    pub periodic_z: bool,
    /// Number of flow degrees of freedom per grid point.
    pub ndof: usize,
    /// Number of grid points in the specified direction.
    pub nr: usize,
    pub nz: usize,
    pub nphi: usize,
    /// Widths of each grid cell.
    pub dr: RVector,
    pub dz: RVector,
    pub r_dphi: RVector,
    pub dphi: f64,
    pub dz_periodic: f64,
    /// Grid point locations.
    pub rgrid: RVector,
    pub zgrid: RVector,
    pub phi_grid: RVector,
    /// Jacobians.
    pub jr: RVector,
    pub jz: RVector,
    /// Inverse Jacobians evaluated at grid points.
    pub ji_r: RVector,
    pub ji_z: RVector,
    pub dphi_inv: f64,
    /// Domain specification.
    pub rmin: f64,
    pub rmax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub phi_min: f64,
    pub phi_max: f64,
    pub delta_phi_domain: f64,
    /// For time step computation due to Yoshizawa model.  These arrays are dimensioned (sr:er,nz) and
    /// is assigned values in subroutine make_grid.
    pub min_grid_size: RMatrix,
    pub l_grid_squared: RMatrix,
    /// Grid spacing vector for Vreman SGS model:
    pub delta_vector: RTensor,
    /// Weights for trapezoidal rule integration:
    pub trap_weight_z: RVector,
    pub trap_weight_r: RVector,
    /// This flag is needed to run an nz = 1 case, i.e., (r, phi) plane case in
    /// parallel.  We fake it by using nz = 2 and suppress z derivatives and smoothing.
    pub suppress_z_derivatives_when_nz_not_1: bool,
    pub nz_actual: usize,
    pub iz_mid: usize,
    pub ir_mid: usize,
}

pub struct QArray {
    pub q: RTensor4,
    pub pressure: RTensor,
}

pub struct SBPDerivatives {
    pub use_sbp_z_derivative: bool,
    pub a_sbp: RVector,
    pub b_sbp: RVector,
    pub c_sbp: RVector,
    pub q_top_sbp: RMatrix,
    pub q_bot_sbp: RMatrix,
}

pub struct Transposes {
    pub q_r_space: RTensor4,
    pub qdot_r_space: RTensor4,
    pub q_phi_space: RTensor4,
    pub qdot_phi_space: RTensor4,
    pub p_phi_space: RTensor,
    pub p_r_space: RTensor,
}

pub struct RK4 {
    pub q_accum: RTensor4,
    pub q_next_arg: RTensor4,
    pub qdot: RTensor4,
}

pub struct ThermalParameters {
    pub gamma: f64,
    pub gm1: f64,
    pub isothermal: bool,
    pub ci_squared_initial: RMatrix,
    pub apply_newtonian_cooling: bool,
    pub tau_newtonian_cooling: f64,
}

pub struct Coordinates {
    pub xgrid: RMatrix,
    pub ygrid: RMatrix,
}

pub struct ThermalSetup {
    pub gamma: f64,
    pub isothermal: bool,
    pub apply_newtonian_cooling: bool,
    pub tau_newtonian_cooling: f64,
    pub ci_squared_initial: RMatrix,
}

pub struct ArtificialPressure {
    pub have_dilatation: bool,
    pub apply_artificial_pressure: bool,
    pub c_ap: f64,
    pub dil: RTensor,
    pub ir_max_ap: usize,
    pub iz_max_ap: usize,
    pub iphi_max_ap: usize,
    pub beta_delta_max: f64,
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
