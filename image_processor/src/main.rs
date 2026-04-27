use env_logger::{Builder, Env};
use std::path::{Path, PathBuf};

use crate::cli::Cli;
use image_processor::{image_io, plugin::Plugin};

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::get_args();

    let img = image_io::load_image(cli.input)?;
    let params = load_params(&cli.params)?;

    let plugin = Plugin::new(&cli.plugin, &cli.plugin_path)?;

    image_io::save_image(cli.output, img)?;

    Ok(())
}

fn load_params(path: &Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}
