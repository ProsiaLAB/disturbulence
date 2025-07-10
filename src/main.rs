use std::env;

use anyhow::Result;
// use fftw::array::AlignedVec;
// use fftw::plan::R2CPlan;
// use fftw::plan::R2CPlanMany;
// use fftw::types::Flag;
// use fftw::types::c64;

use disturbulence::advection;
use disturbulence::config::load_config;
use disturbulence::euler;
use disturbulence::hsbr;
use disturbulence::modes::Config;
use disturbulence::modes::RunType;
use disturbulence::svf;
use disturbulence::tcf;
use disturbulence::user;
use disturbulence::vhb;
use disturbulence::vp;
use disturbulence::vsi;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("examples/vsi.toml");

    let config = load_config(file)?;

    match config.run_type {
        RunType::UserApplication => user::run(),
        RunType::Euler1DTestZ => euler::run(config),
        RunType::VerticalHydrostaticBalance => vhb::run(),
        RunType::SolidBodyRotation => hsbr::run(),
        RunType::SingleVortexFargoTest => svf::run(),
        RunType::VerticalShearInstability => vsi::run(),
        RunType::VortexPair => vp::run(),
        RunType::TaylorCouetteFlow => tcf::run(),
        RunType::AdvectionTest => advection::run(),
        RunType::TimedVsiOutput => vsi::run_timed(),
    };

    // let nphi = 128;
    // let num_transforms = 16;
    // let total_len = nphi * num_transforms;

    // let mut input: AlignedVec<f64> = AlignedVec::new(total_len);
    // let mut output: AlignedVec<c64> = AlignedVec::new((nphi / 2 + 1) * num_transforms);

    // let plan: fftw::plan::R2CPlan64 = R2CPlan::aligned(&[nphi], Flag::ESTIMATE).unwrap();

    // for i in 0..num_transforms {
    //     let in_slice = &mut input[i * nphi..(i + 1) * nphi];
    //     let out_slice = &mut output[i * (nphi / 2 + 1)..(i + 1) * (nphi / 2 + 1)];
    //     plan.r2c(in_slice, out_slice).unwrap();
    // }

    Ok(())
}
