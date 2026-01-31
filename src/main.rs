use std::error::Error;
use twigs::material::Material;
use twigs::mesh::Mesh;
use twigs::run;

fn main() -> Result<(), Box<dyn Error>> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let mesh = Mesh::load("Torsion".to_string(), Material::ALUMINUM_6061)?;
    pollster::block_on(run(&mesh));
    Ok(())
}

