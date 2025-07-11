/// Gravitational constant (cm^3 g^-1 s^-2).
pub const GRAVITATIONAL_CONSTANT: f64 = 6.673e-8;

/// Stefan-Boltzmann constant (erg s^-1 cm^-2 K^-4).
pub const STEFAN_BOTZMANN: f64 = 5.6705e-5;

/// Boltzmann constant (erg K^-1).
pub const BOLTZMANN_CONSTANT: f64 = 1.3807e-16;

/// Solar mass (g).
pub const M_SUN: f64 = 1.989e33;

/// Solar radius (cm).
pub const R_SUN: f64 = 69.57e9;

/// Astronomical unit (cm).
pub const AU: f64 = 1.496e13;

/// Parsec (cm).
pub const PARSEC: f64 = 3.0857e18;

/// Specific Gas constant (cm^2 s^-2 K^-1).
pub const R_GAS: f64 = 3.5871e7;

/// Year (s).
pub const YEAR: f64 = 3.16e7;

/// Mass of atomic hydrogen (g).
pub const M_H: f64 = 1.6735e-24;

/// Mass of atomic hydrogen (g).
pub const M_H2: f64 = 2.0e0 * M_H;

pub mod dof {
    pub const IRHO: usize = 0;
    pub const RMOM: usize = 1;
    pub const ZMOM: usize = 2;
    pub const AMOM: usize = 3;
    pub const ENER: usize = 4;

    /// For second-rank symmetric tensors:
    pub const ZZ: usize = 0;
    pub const ZR: usize = 1;
    pub const PHIZ: usize = 2;
    pub const RR: usize = 3;
    pub const PHIR: usize = 4;
    pub const PHIP: usize = 5;
    pub const PZ: usize = 2;
    pub const PR: usize = 4;
    pub const PP: usize = 5;

    /// For spatial direction indices:
    pub const Z_COMP: usize = 0;
    pub const R_COMP: usize = 1;
    pub const PHI_COMP: usize = 2;
    pub const P_COMP: usize = 2;

    /// For primitive variables (with [IRHO] = 0):
    pub const IUR: usize = 1;
    pub const IUZ: usize = 2;
    pub const IUPHI: usize = 3;
    pub const IP: usize = 4;
}
