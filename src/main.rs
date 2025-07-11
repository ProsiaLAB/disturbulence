use std::env;

use anyhow::Result;

use disturbulence::advection;
use disturbulence::core::RunType;
use disturbulence::euler;
use disturbulence::euler::run;
use disturbulence::hsbr;
use disturbulence::io::load_config;
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
        // RunType::UserApplication => simulate(&config, &user::User),
        RunType::Euler => run(&config),
        // RunType::VerticalHydrostaticBalance => simulate(&config, &vhb::VHB),
        // RunType::SolidBodyRotation => simulate(&config, &hsbr::HSBR),
        // RunType::SingleVortexFargo => simulate(&config, &svf::SVF),
        // RunType::VerticalShearInstability => simulate(&config, &vsi::VSI),
        // RunType::VortexPair => simulate(&config, &vp::VP),
        // RunType::TaylorCouetteFlow => simulate(&config, &tcf::TCF),
        // RunType::Advection => simulate(&config, &advection::Advection),
        // RunType::TimedVsiOutput => simulate(&config, &vsi::VSITimed),
        _ => todo!(),
    };

    Ok(())
}
