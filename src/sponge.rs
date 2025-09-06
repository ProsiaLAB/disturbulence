use extensions::types::RMatrix;

#[derive(Default)]
pub struct Sponge {
    pub apply_sponge: bool,
    pub n_decay_steps: usize,
    pub tau_decay: f64,
    pub sponge_mask: RMatrix,
}
