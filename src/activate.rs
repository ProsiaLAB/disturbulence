use crate::modes::EpsOrTau;

pub struct PadeFilterConfig {
    pub apply_pade_filter: bool,
    pub eps_or_tau: EpsOrTau,
    pub eps_filter: f64,
    pub tau_filter: f64,
    pub filter_relative_to_basic_state: bool,
    pub filtering_interval: usize,
}

pub fn pade_filter(cfg: PadeFilterConfig) {
    todo!()
}
