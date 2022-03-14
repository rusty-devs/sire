use clap::Parser;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::error::{self};
use std::fmt::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
use std::{fs::File, io::BufReader};
use tera::{Context, Tera};
use walkdir::WalkDir;

/// Struct for argument parsing and used by `clap`
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// Directory to use as template source
    #[clap(short, long)]
    pub source_dir: PathBuf,
    /// Directory target to apply changes
    #[clap(short, long)]
    pub destination_dir: PathBuf,
}

impl Config {
    /// Returns a serialized configuration for templating
    ///
    /// # Configuration
    ///
    /// Requires a `config.yml` in the source directory that satisfies
    /// each field in the `TargetConfig` struct
    pub fn load_target_config(&self) -> BoxedResult<TargetConfig> {
        let mut config_file = PathBuf::from(&self.source_dir);
        config_file.push("config.yml");

        match File::open(&config_file) {
            Ok(file) => {
                let reader = BufReader::new(&file);
                match serde_yaml::from_reader(reader) {
                    Ok(conf) => {
                        info!("Loaded target configuration: {}", config_file.display());
                        Ok(conf)
                    }
                    Err(e) => {
                        error!("Cannot deserialize: {}", config_file.display());
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                error!("No configuration found: {}", config_file.display());
                Err(Box::new(e))
            }
        }
    }
}

/// Struct for template variable serialization
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetConfig {
    project_name: String,
    project_description: String,
    license: String,
    version: String,
    email: String,
}

/// Struct for handling core functionality
pub struct App {
    src_dir: PathBuf,
    dest_dir: PathBuf,
    config: TargetConfig,
}

/// If parsing is successful, returns a boolean if `{{sire.project_slug}}`
/// is found in source directory
///
/// # Arguments
///
/// * `path` - A slice of a path to inspect for slug usage
pub fn slug_file_name(path: &Path) -> Option<bool> {
    Some(
        path.file_name()?
            .to_str()?
            .contains("{{sire.project_slug}}"),
    )
}

/// Generic function that recursively searches valid files and directories
/// in a given path and returns a collection of mutable paths
///
/// # Arguments
///
/// * `path` - A generic argument reference to a path to iterate over
pub fn walk_source_dir<P>(path: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    WalkDir::new(path)
        .into_iter()
        .filter_map(|v| v.ok())
        .map(|x| x.path().to_owned())
        .collect()
}

/// Boxed `std::result::Result` type
type BoxedResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Creates the given directory if its missing
///
/// # Argments
///
/// * dir - Slice of a path to check and create if missing
pub fn create_dir_if_missing(dir: &Path) -> BoxedResult<()> {
    if !dir.exists() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

impl App {
    /// The control logic for `sire` functionality
    pub fn run(&self) -> Result {
        create_dir_if_missing(&self.dest_dir).unwrap();

        for item in walk_source_dir(&self.src_dir) {
            debug!("Processing: {}", &item.display());

            if item.is_dir() {
                self.copy_dir_to_dest(&item).ok();
            } else if item.is_file() {
                self.copy_file_to_dest(&item).ok();
            }
        }

        Ok(())
    }

    /// Copy over source directories to specified destination
    /// Raises errors if directories already exist
    ///
    /// # Arguments
    ///
    /// * dir - Slice of a path to create in destination
    fn copy_dir_to_dest(&self, dir: &Path) -> Result {
        let mut dest = PathBuf::new();
        dest.push(&self.dest_dir);

        if slug_file_name(dir).unwrap() {
            dest.push(&self.config.project_name);
        } else if dir == self.src_dir.as_path() {
            // First element in iteration is the `src_dir` itself
            return Ok(());
        } else if let Some(name) = dir.file_name() {
            dest.push(name);
        }

        match create_dir_if_missing(&dest) {
            Ok(_) => info!("Created directory: {}", &dest.display()),
            Err(e) => error!("Cannot create directory: {} {}", &dest.display(), e),
        }

        Ok(())
    }

    /// Copy over source files to specified destination
    ///
    /// # Arguments
    ///
    /// * file - Slice of a path to create in destination
    fn copy_file_to_dest(&self, file: &Path) -> BoxedResult<()> {
        let mut dest = PathBuf::new();

        if let Some(name) = file.file_name() {
            if name == "config.yml" {
                return Ok(());
            }
            dest.push(&self.dest_dir);
            dest.push(name);
        }

        let template: String = fs::read_to_string(&file)?.parse()?;
        let result = Tera::one_off(&template, &Context::from_serialize(&self.config)?, true)?;

        match fs::write(&dest, result) {
            Ok(_) => info!("Created file: {}", &dest.display()),
            Err(e) => {
                error!("Cannot create file: {} {}", &dest.display(), e)
            }
        }

        Ok(())
    }
}

impl From<Config> for App {
    /// Create instance of `App` from `Config`
    fn from(conf: Config) -> App {
        let target_config = conf
            .load_target_config()
            .expect("Configuration failed to load.");
        Self {
            src_dir: conf.source_dir,
            dest_dir: conf.destination_dir,
            config: target_config,
        }
    }
}
