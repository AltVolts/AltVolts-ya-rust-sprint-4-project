use env_logger::{Builder, Env};
use std::path::{Path, PathBuf};

use crate::cli::Cli;
use image_plugin::{image_io, plugin_loader::Plugin};

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::get_args();

    let mut img = image_io::load_image(cli.input)?;
    let params = load_params(&cli.params)?;

    let plugin = Plugin::new(&cli.plugin, &cli.plugin_path)?;

    plugin.process_image(img.width, img.height, &mut img.data, &*params)?;

    image_io::save_image(cli.output, img)?;

    Ok(())
}

fn load_params(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
