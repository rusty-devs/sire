use crate::manifest::{ManifestData, MANIFEST_FILE_NAME};
use clap::Parser;
use log::{debug, error, info};
use std::error;
use std::fmt::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};
use walkdir::WalkDir;
mod manifest;

/// Boxed `std::result::Result` type
type BoxedResult<T> = std::result::Result<T, Box<dyn error::Error>>;

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

/// Struct for handling core functionality
pub struct App {
    source_dir: PathBuf,
    destination_dir: PathBuf,
    manifest: ManifestData,
}

impl Config {
    /// Returns a serialized configuration for templating
    ///
    /// # Configuration
    ///
    /// Requires a `sire.manifest.yml` in the source directory that satisfies
    /// each field in the `ManifestData` struct
    pub fn load_manifest_file(&self) -> BoxedResult<ManifestData> {
        let mut config_file = PathBuf::from(&self.source_dir);
        config_file.push(MANIFEST_FILE_NAME);

        manifest::load_manifest_file(&config_file)
    }
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

/// Creates the given directory if its missing
///
/// # Argments
///
/// * dir_path - Slice of a path to check and create if missing
pub fn create_directory(dir_path: &PathBuf) -> BoxedResult<()> {
    if !dir_path.exists() {
        fs::create_dir(dir_path)?;
    }
    Ok(())
}

impl App {
    /// The control logic for `sire` functionality
    pub fn run(&self) -> Result {
        create_directory(&self.destination_dir).unwrap();

        for item in walk_source_dir(&self.source_dir) {
            debug!("Processing: {}", &item.display());

            if item.is_dir() {
                self.directory_to_destination(&item).unwrap();
            } else if item.is_file() {
                self.file_to_destination(&item).unwrap();
            }
        }

        Ok(())
    }

    /// Construct a new destination directory path and substitute
    /// any `{{sire.project_slug}}` usage in file path
    ///
    /// # Arguments
    ///
    /// * dir_path - Slice of a path
    fn preprocess(&self, dir_path: &Path) -> PathBuf {
        let dest = PathBuf::from_iter([
            &self.destination_dir,
            dir_path.strip_prefix(&self.source_dir).unwrap(),
        ]);
        PathBuf::from(
            dest.as_os_str()
                .to_str()
                .unwrap()
                .replace("{{sire.project_slug}}", &self.manifest.project_name),
        )
    }

    /// Copy over source directories to specified destination
    /// Raises errors if directories already exist
    ///
    /// # Arguments
    ///
    /// * dir_path - Slice of a path
    fn directory_to_destination(&self, dir_path: &PathBuf) -> Result {
        let path = self.preprocess(dir_path);
        if dir_path == self.source_dir.as_path() {
            // First element in the iteration is the `src_dir` itself
            return Ok(());
        }

        match create_directory(&path) {
            Ok(_) => info!("Created directory: {}", &path.display()),
            Err(e) => error!("Cannot create directory: {} {}", &path.display(), e),
        }

        Ok(())
    }

    /// Copy over source files to specified destination
    ///
    /// # Arguments
    ///
    /// * file_path - Slice of a path
    fn file_to_destination(&self, file_path: &PathBuf) -> BoxedResult<()> {
        let path = self.preprocess(file_path);
        if let Some(name) = path.file_name() {
            if name == MANIFEST_FILE_NAME {
                return Ok(());
            }
        }

        let template: String = fs::read_to_string(&file_path)?.parse()?;
        let result = Tera::one_off(&template, &Context::from_serialize(&self.manifest)?, true)?;

        match fs::write(&path, result) {
            Ok(_) => info!("Created file: {}", &path.display()),
            Err(e) => {
                error!("Cannot create file: {} {}", &path.display(), e)
            }
        }

        Ok(())
    }
}

impl From<Config> for App {
    /// Create instance of `App` from `Config`
    fn from(conf: Config) -> App {
        let manifest = conf
            .load_manifest_file()
            .expect("Manifest file failed to load.");
        Self {
            source_dir: conf.source_dir,
            destination_dir: conf.destination_dir,
            manifest,
        }
    }
}
