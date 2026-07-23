const CONFIG_PATH: &str = "~/.config/cantor/config.toml";
const ALTERNATIVE_CONFIG_PATH: &str = "../example.toml";

use std::path::{Path, PathBuf};
use std::fs;
use serde::Deserialize;
use std::collections::HashMap;
// serde here is used to deserialize the toml file into
// a struct. goddamit why caant they read from a txt file :((()))

#[derive(Debug, Deserialize)]
enum ModulePosition {
    left,
    right,
    center,
}

// firstly construct the bar
// then do everything else with modules
#[derive(Debug, Deserialize)]
pub struct Config {
    enabled: EnabledModules,
    general: GeneralSettings,
    modules: ModulesConfig,
}

#[derive(Debug, Deserialize)]
struct EnabledModules {
    battery: bool,
    time: bool,
    workspaces: bool,
    cpu: bool,
    memory: bool,
}

#[derive(Debug, Deserialize)]
struct GeneralSettings {
    color: String,
    height: usize,
    border: bool,
    border_color: String,
}

#[derive(Debug, Deserialize)]
struct ModulesConfig {
    battery: ModuleSettings,
    time: ModuleSettings,
    workspaces: WorkspacesSettings,
    cpu: ModuleSettings,
    memory: ModuleSettings,
}

#[derive(Debug, Deserialize)]
struct ModuleSettings {
    position: ModulePosition,
    color: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct WorkspacesSettings {
    position: ModulePosition,
    color: String,
    icons: HashMap<usize, String>,  // my icons
}

pub fn read_the_config() -> Config {
    let mut config: PathBuf = PathBuf::new();
    let configs = [ Path::new(&CONFIG_PATH), Path::new(&ALTERNATIVE_CONFIG_PATH) ];
    // bitch ass search
    match configs.iter().find(|config| config.exists()) {
        Some(found) => {
            println!("Using config file under: {:?}", found);
            config = (&(**found)).to_path_buf();
        },
        None => panic!("None of the required config files exists. Please, create a config file!")
    }
    let contents = fs::read_to_string(&config)
        .unwrap_or_else(|e| panic!("Failed to read the config {:?} contents: {}", config, e));
    // return plain setup
    toml::from_str(&contents)
        .unwrap_or_else(|e| panic!("Failed to parse the config {:?}: {}", config, e))
}

