pub mod filters;
pub mod structs;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{BufReader, Result, Write},
    path::Path,
};

/// Open the config file at `path` and deserialise it into a config struct
pub fn read_config(path: impl AsRef<Path>) -> Result<structs::Config> {
    if !path.as_ref().exists() {
        create_dir_all(path.as_ref().parent().expect("Invalid config directory"))?;
        write_config(&path, &structs::Config::default())?;
    }

    let config_file = BufReader::new(File::open(&path)?);
    let mut config: structs::Config = serde_json::from_reader(config_file)?;

    config
        .profiles
        .iter_mut()
        .for_each(structs::Profile::backwards_compat);

    Ok(config)
}

/// Serialise `config` and write it to the config file at `path`
///
/// This function implements dirty tracking - it only writes if the config has actually changed
pub fn write_config(path: impl AsRef<Path>, config: &structs::Config) -> Result<()> {
    // Serialize the config to a string
    let new_content = serde_json::to_string_pretty(config)?;

    // Check if file exists and read current content
    let path = path.as_ref();
    let should_write = if path.exists() {
        // Compare with existing content
        match read_to_string(path) {
            Ok(existing_content) => existing_content != new_content,
            Err(_) => true, // If we can't read, write anyway
        }
    } else {
        true // File doesn't exist, definitely write
    };

    // Only write if content has changed
    if should_write {
        let mut config_file = File::create(path)?;
        config_file.write_all(new_content.as_bytes())?;
    }

    Ok(())
}
