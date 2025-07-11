use crate::types::{CMatrix, RVector, UVector};

#[derive(Default)]
pub struct FargoShift {
    pub apply_fargo: bool,
    pub apply_fargo_extra_operator: bool,
    pub integer_shifts: bool,
    pub apply_plotting_shift: bool,
    pub add_fargo_extra_operator_now: bool,
    pub apply_fargo_this_step: bool,
    pub have_omega_bar: bool,
    pub omega_bar: RVector,
    pub omega_fargo: RVector,
    pub uphi_fargo_subtract: RVector,
    pub nshift: UVector,
    pub inew_phi: UVector,
    pub d_omega_bar_dr: RVector,
    pub fargo_factor: RVector,
    pub fargo_factor_over_r: RVector,
    pub complex_shift_factor: CMatrix,
    pub omega_plotting_shift: f64,
    pub t_of_previous_shift: f64,
    pub revolution_over_delta_phi_domain: f64,
    pub fft_has_been_initialized: bool,
}
