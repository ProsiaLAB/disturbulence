use ndarray::s;

use crate::activate::{self, PadeFilterConfig};
use crate::mesh::{MeshSetup, set_up_domain_mesh_and_partition};
use crate::modes::{BCType, Config, EpsOrTau, ProblemType};
use crate::setup::{BCSetup, ThermalSetup, set_up_boundary_conditions, set_up_thermal_parameters};
use crate::types::{RMatrix, RVector};

pub fn run(mut cfg: Config) {
    println!("Running Euler 1D test");

    let mut ci_squared_initial = RMatrix::zeros((cfg.nz, 1));

    let rmin = 1.0;
    let rmax = 1.0;
    let phi_min = 0.0;
    let phi_max = 0.0;
    let nr = 1;
    let nphi = 1;
    let suppress_z_derivatives_when_nz_not_1 = false;

    let (zmin, zmax) = match cfg.problem_type {
        ProblemType::AcousticPulse => (0.0, 4.0),
        ProblemType::ShockTube => (0.0, 2.0),
        ProblemType::DensityWaves => (-5.0, 5.0),
    };

    let gamma = 1.4;
    let gm1 = gamma - 1.0;

    // Set up boundary conditions:
    cfg.bc.rmin = BCType::Null;
    cfg.bc.rmax = BCType::Null;

    let eps: f64;
    let z0: f64;
    let sigma: f64;
    let p0: f64;
    let rho0: f64;

    if cfg.problem_type == ProblemType::AcousticPulse {
        cfg.apply_artificial_pressure = false;
        cfg.apply_pade_filter = false;
        if cfg.periodic_z {
            cfg.bc.zmin = BCType::Periodic;
            cfg.bc.zmax = BCType::Periodic;
        } else {
            cfg.bc.zmin = BCType::NonReflective;
            cfg.bc.zmax = BCType::NonReflective;
            cfg.balanced = true;
        }
        eps = 0.005;
        z0 = 2.0;
        sigma = 0.04 * (zmax - zmin);
        p0 = if cfg.isothermal { 1.0 } else { 1.0 / gamma };
        rho0 = 1.0;
    } else {
        // `ProblemType::ShockTube` or `ProblemType::DensityWaves`:
        cfg.isothermal = false;
        cfg.periodic_z = false;
        cfg.bc.zmin = BCType::ZDirichlet;
        cfg.bc.zmax = BCType::ZDirichlet;
    };

    let setup = MeshSetup {
        rmin,
        rmax,
        zmin,
        zmax,
        phi_min,
        phi_max,
        nr,
        nz: cfg.nz,
        nphi,
        suppress_z_derivatives_when_nz_not_1,
        periodic_z: cfg.periodic_z,
        stretched_r: false,
        stretched_z: false,
        r0: 0.0,
        nr_u: 0,
        z0: 0.0,
        nz_u: 0,
    };

    set_up_domain_mesh_and_partition(setup);

    let mut c_sound_rmin: f64 = 0.0;
    let mut c_sound_rmax: f64 = 0.0;

    if cfg.isothermal {
        // Not used here
        c_sound_rmin = 1.0;
        c_sound_rmax = 1.0;

        ci_squared_initial.slice_mut(s![.., 0]).fill(1.0);
    }

    cfg.apply_newtonian_cooling = false;
    let tau_newtonian_cooling = 0.0;

    let thermal_setup = ThermalSetup {
        gamma,
        isothermal: cfg.isothermal,
        apply_newtonian_cooling: cfg.apply_newtonian_cooling,
        tau_newtonian_cooling,
        ci_squared_initial,
    };

    set_up_thermal_parameters(thermal_setup);

    let d_ci_dr_inner = RVector::zeros(1);
    let d_ci_dr_outer = RVector::zeros(1);

    let bc_setup = BCSetup {
        rmin: rmin_bc,
        rmax: rmax_bc,
        zmin: zmin_bc,
        zmax: zmax_bc,
        balanced,
        d_ci_dr_inner,
        d_ci_dr_outer,
        c_sound_rmin,
        c_sound_rmax,
        isothermal: cfg.isothermal,
    };

    set_up_boundary_conditions(bc_setup);

    let mut eps_or_tau = EpsOrTau::Eps;

    if cfg.apply_pade_filter {
        eps_or_tau = EpsOrTau::Eps;

        let pf_cfg = PadeFilterConfig {
            apply_pade_filter: cfg.apply_pade_filter,
            eps_or_tau,
            eps_filter: cfg.eps_filter,
            tau_filter: cfg.tau_filter,
            filter_relative_to_basic_state: false,
            filtering_interval: cfg.filtering_interval,
        };

        activate::pade_filter(pf_cfg);
    }
}
