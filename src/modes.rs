use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RunType {
    UserApplication,
    Euler1DTestZ,
    VerticalHydrostaticBalance,
    SolidBodyRotation,
    SingleVortexFargoTest,
    VerticalShearInstability,
    VortexPair,
    TaylorCouetteFlow,
    AdvectionTest,
    TimedVsiOutput,
}

#[derive(Debug, Deserialize, Default)]
pub struct VerticalShearInstabilityConfig {
    #[serde(default)]
    pub plot_many_horizontal_planes: bool,
    #[serde(default)]
    pub time_interval_for_many_horizontal_plots: f64,
    #[serde(default)]
    pub restart: bool,
    #[serde(default)]
    pub file_version: FileVersion,
    #[serde(default)]
    pub perturb: bool,
    #[serde(default)]
    pub isothermal: bool,
    #[serde(default)]
    pub nr: usize,
    #[serde(default)]
    pub nz: usize,
    #[serde(default)]
    pub nphi: usize,
    #[serde(default)]
    pub phi_max_over_pi: f64,
    #[serde(default)]
    pub cfl: f64,
    #[serde(default)]
    pub dt_min: f64,
    #[serde(default)]
    pub nsteps: usize,
    #[serde(default)]
    pub tecplot_interval: usize,
    #[serde(default)]
    pub output_profiles: bool,
    #[serde(default)]
    pub profiles_interval: usize,
    #[serde(default)]
    pub output_phi_reynolds_averages: bool,
    #[serde(default)]
    pub time_interval_for_phi_reynolds_averages: f64,
    #[serde(default)]
    pub fluctuation_kinetic_energy_interval: usize,
    #[serde(default)]
    pub save_interval: usize,
    #[serde(default)]
    pub wavy_perturbation: bool,
    #[serde(default)]
    pub zmax_over_h0: f64,
    #[serde(default)]
    pub use_rsize_for_domain: bool,
    #[serde(default)]
    pub rsize_over_h0: f64,
    #[serde(default)]
    pub rmin_over_h0: f64,
    #[serde(default)]
    pub n_waves_in_r: usize,
    #[serde(default)]
    pub use_manger_p: bool,
    #[serde(default)]
    pub apply_newtonian_cooling: bool,
    #[serde(default)]
    pub tau_cooling_over_t_kepler_at_mid_radius: f64,
    #[serde(default)]
    pub apply_pade_filter: bool,
    #[serde(default)]
    pub eps_or_tau: EpsOrTau,
    #[serde(default)]
    pub eps_filter: f64,
    #[serde(default)]
    pub tau_filter: f64,
    #[serde(default)]
    pub apply_artificial_pressure: bool,
    #[serde(default)]
    pub c_ap: f64,
    #[serde(default)]
    pub apply_viscosity: bool,
    #[serde(default)]
    pub viscosity_type: ViscosityType,
    #[serde(default)]
    pub c_ddsv: f64,
    #[serde(default)]
    pub name_using_step: bool,
    #[serde(default)]
    pub bc: BoundaryConditions,
    #[serde(default)]
    pub balanced: bool,
    #[serde(default)]
    pub apply_fargo_trick: bool,
    #[serde(default)]
    pub apply_fargo_correction: bool,
    #[serde(default)]
    pub sponge: SpongeSettings,
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

#[derive(Debug, Deserialize, Default)]
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
