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
