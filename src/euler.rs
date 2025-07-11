use ndarray::s;

use crate::core::{
    BCType, BoundaryConditionData, Config, EpsOrTau, ProblemType, ThermalParameters,
};
use crate::core::{Grid, QArray, RK4, StretchedMesh, Transposes};
use crate::fargo::FargoShift;
use crate::gravity::Gravity;
use crate::grid::make_grid;
use crate::pade::PadeFilter;
use crate::pade::set_up_pade_coefficients;
use crate::processing::get_xy_coordinates_of_grid;
use crate::sponge::Sponge;
use crate::types::{RMatrix, RTensor, RTensor4, RVector};
use crate::viscosity::Viscosity;

pub fn run(cfg: &Config) {
    println!("Running Euler...");

    const NDOF: usize = 5;

    let (zmin, zmax) = match cfg.problem_type {
        ProblemType::AcousticPulse => (0.0, 4.0),
        ProblemType::ShockTube => (0.0, 2.0),
        ProblemType::DensityWaves => (-5.0, 5.0),
    };

    let isothermal = match cfg.problem_type {
        ProblemType::AcousticPulse => cfg.isothermal,
        _ => false,
    };

    let mut ci_squared_initial = RMatrix::zeros((cfg.nz, 1));

    if isothermal {
        ci_squared_initial.slice_mut(s![.., 0]).fill(1.0);
    }

    let stretched_mesh = StretchedMesh::default();

    let grid = Grid {
        rmin: 1.0,
        rmax: 1.0,
        zmin,
        zmax,
        phi_min: 0.0,
        phi_max: 0.0,
        nr: 1,
        nz: cfg.nz,
        nphi: 1,
        suppress_z_derivatives_when_nz_not_1: false,
        periodic_z: cfg.periodic_z,
        nz_actual: cfg.nz, // As `suppress_z_derivatives_when_nz_not_1` is false.
        ..Default::default()
    };

    let qarr = QArray {
        q: RTensor4::zeros((grid.nr, grid.nphi, grid.nz, NDOF)),
        pressure: RTensor::zeros((grid.nr, grid.nphi, grid.nz)),
    };

    let rk4 = RK4 {
        q_accum: RTensor4::zeros((grid.nr, grid.nphi, grid.nz, NDOF)),
        q_next_arg: RTensor4::zeros((grid.nr, grid.nphi, grid.nz, NDOF)),
        qdot: RTensor4::zeros((grid.nr, grid.nphi, grid.nz, NDOF)),
    };

    let tranposes = Transposes {
        q_r_space: RTensor4::zeros((grid.nphi, grid.nz, NDOF, grid.nr)),
        qdot_r_space: RTensor4::zeros((grid.nphi, grid.nz, NDOF, grid.nr)),
        p_r_space: RTensor::zeros((grid.nphi, grid.nz, grid.nr)),
        q_phi_space: RTensor4::zeros((grid.nr, grid.nz, NDOF, grid.nphi)),
        qdot_phi_space: RTensor4::zeros((grid.nr, grid.nz, NDOF, grid.nphi)),
        p_phi_space: RTensor::zeros((grid.nr, grid.nz, grid.nphi)),
    };

    make_grid();
    set_up_pade_coefficients();

    if grid.nphi != 1 {
        get_xy_coordinates_of_grid();
    }

    let fargo = FargoShift::default();

    let pade_filter = PadeFilter::default();

    let viscosity = Viscosity::default();

    let sponge = Sponge::default();

    let gravity = Gravity::default();

    let thermal_parameters = ThermalParameters {
        gamma: 1.4,
        gm1: 1.4 - 1.0,
        isothermal,
        apply_newtonian_cooling: false,
        tau_newtonian_cooling: 0.0,
        ci_squared_initial,
    };

    let (rmin_bc, rmax_bc) = (BCType::Null, BCType::Null);

    let (mut zmin_bc, mut zmax_bc) = match cfg.problem_type {
        ProblemType::AcousticPulse => {
            if cfg.periodic_z {
                (BCType::Periodic, BCType::Periodic)
            } else {
                (BCType::NonReflective, BCType::NonReflective)
            }
        }
        _ => (BCType::ZDirichlet, BCType::ZDirichlet),
    };

    let balanced = match cfg.problem_type {
        ProblemType::AcousticPulse => cfg.periodic_z,
        _ => false,
    };

    let (c_sound_rmin, c_sound_rmax) = if isothermal { (1.0, 1.0) } else { (0.0, 0.0) };

    let (c_sound_rmin_used, c_sound_rmax_used) = if rmin_bc == BCType::ViscousWall && !isothermal {
        (c_sound_rmin, c_sound_rmax)
    } else {
        (0.0, 0.0)
    };

    let q_left_z_bc = if zmin_bc == BCType::ZDirichlet {
        RVector::zeros(NDOF)
    } else {
        RVector::default(NDOF)
    };

    let q_right_z_bc = if zmax_bc == BCType::ZDirichlet {
        RVector::zeros(NDOF)
    } else {
        RVector::default(NDOF)
    };

    if grid.periodic_z && zmin_bc != BCType::Periodic {
        zmin_bc = BCType::Periodic;
    }

    if grid.periodic_z && zmax_bc != BCType::Periodic {
        zmax_bc = BCType::Periodic;
    }

    let (d_ci_dr_inner, d_ci_dr_outer) = if isothermal {
        (RVector::zeros(1), RVector::zeros(1))
    } else {
        (RVector::default(1), RVector::default(1))
    };

    let bc_data = BoundaryConditionData {
        rmin: rmin_bc,
        rmax: rmax_bc,
        zmin: zmin_bc,
        zmax: zmax_bc,
        balanced,
        d_ci_dr_inner,
        d_ci_dr_outer,
        c_sound_rmin: c_sound_rmin_used,
        c_sound_rmax: c_sound_rmax_used,
        q_left_z_bc,
        q_right_z_bc,
        ..Default::default()
    };

    // if cfg.apply_pade_filter {
    //     let eps_or_tau = EpsOrTau::Eps;
    //     let pade_filter =

    // }
}
