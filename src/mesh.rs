pub struct MeshSetup {
    pub rmin: f64,
    pub rmax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub phi_min: f64,
    pub phi_max: f64,
    pub nr: usize,
    pub nz: usize,
    pub nphi: usize,
    pub suppress_z_derivatives_when_nz_not_1: bool,
    pub periodic_z: bool,
    pub stretched_r: bool,
    pub stretched_z: bool,
    pub r0: f64,
    pub nr_u: usize,
    pub z0: f64,
    pub nz_u: usize,
}

pub fn set_up_domain_mesh_and_partition(setup: MeshSetup) {
    todo!()
}
