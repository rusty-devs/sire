//! Primitives for interacting with sire manifest files
//!
//! Manifest files are YAML formatted text files that contain the
//! parameters needed to apply the sire framework against a template
//! project. This file is expected to exist in the root folder of
//! the template project and must be named sire.manifest.yml
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::error;
use std::path::PathBuf;
use std::{fs::File, io::BufReader};

type BoxedResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Default file name for manifest file
pub const MANIFEST_FILE_NAME: &str = "sire.manifest.yml";

/// Struct for template variable serialization
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestData {
    pub project_name: String,
    pub project_description: String,
    pub repository: String,
    pub homepage: String,
    pub full_name: String,
    pub license: String,
    pub version: String,
    pub email: String,
    #[serde(flatten)]
    pub extras: HashMap<String, Value>,
}

/// Returns template configuration data loaded from a manifest file
///
/// # Arguments
///
/// * `file_path` - path to the manifest file to be parsed
pub fn load_manifest_file(file_path: &PathBuf) -> BoxedResult<ManifestData> {
    match File::open(&file_path) {
        Ok(file) => {
            let reader = BufReader::new(&file);
            match serde_yaml::from_reader(reader) {
                Ok(conf) => {
                    info!("Loaded manifest: {}", file_path.display());
                    Ok(conf)
                }
                Err(e) => {
                    error!("Cannot deserialize manifest: {}", file_path.display());
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            error!("No manifest file found: {}", file_path.display());
            Err(Box::new(e))
        }
    }
}

//-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//                              UNIT TESTS
//-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_manifest_success() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "test_data",
            "manifest_files",
            "complete_sample.yml",
        ]
        .iter()
        .collect();
        let result = load_manifest_file(&path).unwrap();

        assert_eq!(result.project_name, "test-project");
        assert_eq!(result.project_description, "A description.");
        assert_eq!(result.repository, "https://github.com/namespace/repo");
        assert_eq!(result.homepage, "http://example.com");
        assert_eq!(result.full_name, "Jane Doe");
        assert_eq!(result.license, "MIT");
        assert_eq!(result.version, "0.1.0");
        assert_eq!(result.email, "foo.bar@example.com");
        let extras = &result.extras;
        let organization = extras["extras"][Value::String(String::from("organization"))]
            .as_str()
            .unwrap();
        assert_eq!(organization, "Fizz Buzz");

        let build_mode = extras["extras"][Value::String(String::from("build_mode"))]
            .as_sequence()
            .unwrap();
        assert_eq!(build_mode.len(), 2);
        let first = build_mode[0].as_str().unwrap();
        assert_eq!(first, "foo");
        let second = build_mode[1].as_str().unwrap();
        assert_eq!(second, "bar");

        let debug = extras["extras"][Value::String(String::from("debug"))]
            .as_bool()
            .unwrap();
        assert!(debug);
    }

    #[test]
    #[should_panic]
    fn missing_project_name() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "test_data",
            "manifest_files",
            "missing_project_name.yml",
        ]
        .iter()
        .collect();
        let temp = load_manifest_file(&path);
        temp.unwrap();
    }

    #[test]
    fn parse_simple_manifest_success() {
        let path = [
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "test_data",
            "manifest_files",
            "simple_manifest.yml",
        ]
        .iter()
        .collect();
        let result = load_manifest_file(&path).unwrap();

        assert_eq!(result.project_name, "test-project");
        assert_eq!(result.project_description, "A description.");
        assert_eq!(result.repository, "https://github.com/namespace/repo");
        assert_eq!(result.homepage, "http://example.com");
        assert_eq!(result.full_name, "Jane Doe");
        assert_eq!(result.license, "MIT");
        assert_eq!(result.version, "0.1.0");
        assert_eq!(result.email, "foo.bar@example.com");
        assert_eq!(result.extras.len(), 0);
    }
}
