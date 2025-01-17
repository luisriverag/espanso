use espanso_path::Paths;

use espanso_engine::process::ConfigPathProvider;

pub struct ConfigPathProviderAdapter<'a> {
  paths: &'a Paths,
}

impl<'a> ConfigPathProviderAdapter<'a> {
  pub fn new(paths: &'a Paths) -> Self {
    Self { paths }
  }
}

impl ConfigPathProvider for ConfigPathProviderAdapter<'_> {
  fn get_config_path(&self) -> &std::path::Path {
    &self.paths.config
  }
}
