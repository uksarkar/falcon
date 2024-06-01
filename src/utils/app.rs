use directories::ProjectDirs;
use std::sync::OnceLock;

use crate::constants::{FALCON_APPLICATION, FALCON_ORGANIZATION, FALCON_QUALIFIER};

pub fn app_config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AppConfig::load_app_configs()
            .unwrap_or_else(|ex| panic!("Unable to extract application configs - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct AppConfig {
    pub CONFIG_DIR: String,
    pub DATA_DIR: String,
    pub CACHE_DIR: String,
}

impl AppConfig {
    fn load_app_configs() -> Result<AppConfig, String> {
        if let Some(proj_dirs) =
            ProjectDirs::from(FALCON_QUALIFIER, FALCON_ORGANIZATION, FALCON_APPLICATION)
        {
            return Ok(AppConfig {
                CACHE_DIR: proj_dirs.cache_dir().to_str().unwrap().to_string(),
                CONFIG_DIR: proj_dirs.config_dir().to_str().unwrap().to_string(),
                DATA_DIR: proj_dirs.data_dir().to_str().unwrap().to_string(),
            });
        }

        Err("Unable to get OS information".into())
    }
}
