use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::modes::{
    AdvectionConfig, Euler1DConfig, HSBRConfig, SVFConfig, TCFConfig, TimedVSIConfig, UserConfig,
    VHBConfig, VPConfig, VSIConfig,
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Config {
    UserApplication {
        user_application: UserConfig,
    },
    Euler1D {
        euler1d: Euler1DConfig,
    },
    VerticalHydrostaticBalance {
        vertical_hydrostatic_balance: VHBConfig,
    },
    SolidBodyRotation {
        solid_body_rotation: HSBRConfig,
    },
    SingleVortexFargoTest {
        single_vortex_fargo_test: SVFConfig,
    },
    VerticalShearInstability {
        vertical_shear_instability: VSIConfig,
    },
    VortexPair {
        vortex_pair: VPConfig,
    },
    TaylorCouetteFlow {
        taylor_couette_flow: TCFConfig,
    },
    AdvectionTest {
        advection_test: AdvectionConfig,
    },
    TimedVsiOutput {
        timed_vsi_output: TimedVSIConfig,
    },
}

pub fn load_config(path: &str) -> Result<Config> {
    let toml_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}
