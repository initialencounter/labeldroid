#![deny(clippy::all)]

use napi_derive::napi;
use labeldroid_types::config::Config;
use share::config::ConfigManager;

#[napi]
pub fn get_config() -> Config {
  ConfigManager::get_config()
}

#[napi]
pub fn set_config(config: Config) {
  ConfigManager::save_config(&config);
}
