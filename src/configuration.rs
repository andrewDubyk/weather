use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

/// Name of the configuration folder
const CONFIG_DIR_NAME: &str = "weather_configs";
/// Name of the file with current active provider
const ACTIVE_PROVIDER_FILE_NAME: &str = "active";

/// Returns uniqe id based on user information
fn get_user_config_id() -> String {
    format!("{}_{}", whoami::hostname(), whoami::username())
}

/// Returns path to configuration directory for user
fn get_user_config_dir_path() -> Result<path::PathBuf, String> {
    match env::current_exe() {
        Ok(path) => match path.parent() {
            Some(path) => Ok(path.join(CONFIG_DIR_NAME).join(get_user_config_id())),
            None => Err("Failed to get configuration directory path".to_string()),
        },
        Err(e) => Err(format!("Failed to get exectable path : {}", e)),
    }
}

/// Returns path to configuration file with active provider name
fn get_active_provider_config_path() -> Result<path::PathBuf, String> {
    let config_dir = match get_user_config_dir_path() {
        Ok(path) => path,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(config_dir.join(ACTIVE_PROVIDER_FILE_NAME))
}

/// Returns path to configuration file for choosen provider.
/// Also validate if file exists.
/// # Arguments
///
/// * `provider` - String reference wth provider name
///
fn get_provider_config_path(provider: &str) -> Result<path::PathBuf, String> {
    let provider_config_path = match get_user_config_dir_path() {
        Ok(path) => path.join(provider),
        Err(e) => {
            return Err(e);
        }
    };

    if !path::Path::new(&provider_config_path).exists() {
        return Err(format!(
            "Configuration for provider {} was not found",
            provider
        ));
    }

    Ok(provider_config_path)
}

/// Update configuration file (create if not exists and override)
///
/// # Arguments
///
/// * `file_path` - PathBuf object with path to file which sould be created/updated
/// * `data` - String slice data to write to file
///
fn update_config(file_path: &path::Path, data: &str) -> io::Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?
        .write_all(data.as_bytes())?;

    Ok(())
}

fn read_config(file_path: &path::Path) -> Result<String, String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            return Err(format!("Failed to read configuration from file : {}", e));
        }
    }
}

/// Returns name of currently configured provider service
fn get_active_provider() -> Result<String, String> {
    let provider_name = match get_active_provider_config_path() {
        Ok(path) => match read_config(&path) {
            Ok(data) => data,
            Err(e) => return Err(e),
        },
        Err(e) => {
            return Err(format!(
                "Failed to get active provider configuration path : {}",
                e
            ));
        }
    };

    Ok(provider_name)
}

/// Returns API_KEY for selected provider
///
/// # Arguments
///
/// * `provider` - String reference wth provider name
///
fn get_provider_api_key(provider: &str) -> Result<String, String> {
    let provider_api_key = match get_provider_config_path(provider) {
        Ok(path) => match read_config(&path) {
            Ok(data) => data,
            Err(e) => return Err(e),
        },
        Err(e) => {
            return Err(format!(
                "Failed to get provider api_key configuration path : {}",
                e
            ));
        }
    };

    Ok(provider_api_key)
}

/// Configure credentials for provider and set active provider
///
/// # Arguments
///
/// * `provider` - Provider name
/// * `api_key` - API_KEY for provider API
///
pub fn configure_provider(provider: &str, api_key: Option<&str>) -> Result<(), String> {
    let config_dir = match get_user_config_dir_path() {
        Ok(path) => path,
        Err(e) => {
            return Err(e);
        }
    };

    match fs::create_dir_all(&config_dir) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Failed to create configuration directory : {}", e));
        }
    }

    match fs::create_dir_all(&config_dir) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("Failed to create configuration dir : {}", e));
        }
    }

    if let Some(api_key) = api_key {
        match update_config(&config_dir.join(provider), api_key) {
            Ok(_) => {
                println!("Successfully update provider API_KEY configuration");
            }
            Err(e) => {
                return Err(format!(
                    "Failed to set API_KEY for provider configuration : {}",
                    e
                ));
            }
        }
    }

    match update_config(&config_dir.join(ACTIVE_PROVIDER_FILE_NAME), provider) {
        Ok(_) => {
            println!("Active provider : {}", provider);
        }
        Err(e) => {
            return Err(format!(
                "Failed to set active provider configuration : {}",
                e
            ));
        }
    }

    Ok(())
}

/// Returns provider information which is configured by program caller (user).
/// If success - returns tuple with currently configured provider name and respective API_key.
/// Otherwise - returns error details
pub fn get_provider_info() -> Result<(String, String), String> {
    let provider = match get_active_provider() {
        Ok(provider) => provider,
        Err(e) => return Err(e),
    };
    let api_key = match get_provider_api_key(&provider) {
        Ok(api_key) => api_key,
        Err(e) => return Err(e),
    };

    Ok((provider, api_key))
}
