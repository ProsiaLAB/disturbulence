use crate::core::EpsOrTau;

#[derive(Default)]
pub struct PadeFilter {
    pub apply_pade_filter: bool,
    pub filter_relative_to_basic_state: bool,
    pub eps_or_tau: EpsOrTau,
    pub eps_filter: f64,
    pub tau_filter: f64,
    pub filtering_interval: usize,
}

pub fn set_up_pade_coefficients() {
    todo!()
}
