use serde::Deserialize;

use extensions::types::{RMatrix, RTensor, RTensor4, RVector};

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RunType {
    UserApplication,
    #[default]
    Euler,
    VerticalHydrostaticBalance,
    SolidBodyRotation,
    SingleVortexFargo,
    VerticalShearInstability,
    VortexPair,
    TaylorCouetteFlow,
    Advection,
    TimedVsiOutput,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub run_type: RunType,
    #[serde(default)]
    pub nz: usize,
    #[serde(default)]
    pub problem_type: ProblemType,
    #[serde(default)]
    pub n_output_steps: usize,
    #[serde(default)]
    pub dt_output: f64,
    #[serde(default)]
    pub cfl: f64,
    #[serde(default)]
    pub isothermal: bool,
    #[serde(default)]
    pub periodic_z: bool,
    #[serde(default)]
    pub apply_pade_filter: bool,
    #[serde(default)]
    pub eps_filter: f64,
    #[serde(default)]
    pub apply_artificial_pressure: bool,
    #[serde(default)]
    pub c_ap: f64,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProblemType {
    #[default]
    AcousticPulse,
    ShockTube,
    DensityWaves,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FileVersion {
    Old,
    #[default]
    New,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EpsOrTau {
    #[default]
    Eps,
    Tau,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ViscosityType {
    Laminar,
    Molecular,
    #[default]
    MoinLes,
    Vreman,
    Ddsv,
    MoinDdsv,
    VremanDdsv,
}

#[derive(Debug, Deserialize, Default)]
pub struct BoundaryConditions {
    pub rmin: BCType,
    pub rmax: BCType,
    pub zmin: BCType,
    pub zmax: BCType,
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BCType {
    Null,
    NonReflective,
    #[default]
    ZeroNormalMomentum,
    CassenMoosman,
    ZeroNormalMomentumIfSubsonic,
    Outflow,
    Periodic,
    ViscousWall,
    ZeroShearStress,
    ZDirichlet,
    HoldBasicState,
}

#[derive(Debug, Deserialize, Default)]
pub struct SpongeSettings {
    pub apply_sponge: bool,
    #[serde(rename = "sponge_type")]
    pub sponge_type: SpongeType,
    pub rho1: f64,
    pub rho2: f64,
    pub d1: f64,
    pub d2: f64,
    pub n_decay_steps: usize,
    pub tau_decay: f64,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SpongeType {
    BsdNul,
    DrwNul,
    #[default]
    DawNul,
    BsdDrm,
    DrwBsd,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Marcus,
    Dong,
    Axial,
    Meyer,
    Moser,
    #[default]
    DongCounterRotating,
    LaminarMoser,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SubMode {
    Meyer,
    #[serde(rename = "ds_narrow")]
    DSNarrow,
    #[serde(rename = "ds_wide")]
    #[default]
    DSWide,
}

pub struct Control {
    pub istep: usize,
    pub nsteps: usize,
    pub t: f64,
    pub cfl: f64,
    // pub file_state: FileState,
}

#[derive(Default)]
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

// impl Grid {
//     pub fn from_config(cfg: &Config) -> Self {
//         let grid = Grid::default();
//     }
// }

#[derive(Default)]
pub struct StretchedMesh {
    pub stretched_r: bool,
    pub stretched_z: bool,
    pub r0: f64,
    pub z0: f64,
    pub nr_u: usize,
    pub nz_u: usize,
}

#[derive(Default)]
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

#[derive(Default)]
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

#[derive(Default)]
pub struct ThermalParameters {
    pub gamma: f64,
    pub gm1: f64,
    pub isothermal: bool,
    pub ci_squared_initial: RMatrix,
    pub apply_newtonian_cooling: bool,
    pub tau_newtonian_cooling: f64,
}

#[derive(Default)]
pub struct BoundaryConditionData {
    pub rmin: BCType,
    pub rmax: BCType,
    pub zmin: BCType,
    pub zmax: BCType,
    pub balanced: bool,
    pub omega_rmin: f64,
    pub omega_rmax: f64,
    pub uz_rmin: f64,
    pub uz_rmax: f64,
    pub c_sound_rmin: f64,
    pub c_sound_rmax: f64,
    pub specify_viscous_wall_conditions_was_called: bool,
    pub d_ci_dr_inner: RVector,
    pub d_ci_dr_outer: RVector,
    pub q_left_z_bc: RVector,
    pub q_right_z_bc: RVector,
}

pub struct Coordinates {
    pub xgrid: RMatrix,
    pub ygrid: RMatrix,
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
