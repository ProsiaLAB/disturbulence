use disturbulence::config::ConfigFile;
use disturbulence::config::load_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("examples/vsi.toml")?;

    match config {
        ConfigFile::VerticalShearInstability {
            vertical_shear_instability,
        } => {
            // run_vsi(vertical_shear_instability);
            dbg!(vertical_shear_instability);
        } // Other variants...
    }

    Ok(())
}
