use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Result;
use std::io;
use std::{
    fs,
    path::{Path, PathBuf},
};
use std::{fs::File, io::BufReader};
use tera::{Context, Tera};
use walkdir::WalkDir;

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
    pub fn load_target_config(&self) -> TargetConfig {
        let mut config_file = PathBuf::from(&self.source_dir);
        config_file.push("config.yml");
        let file = File::open(&config_file).unwrap();
        let reader = BufReader::new(&file);
        serde_yaml::from_reader(reader).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetConfig {
    project_name: String,
    project_description: String,
    license: String,
    version: String,
    email: String,
}

pub struct App {
    src_dir: PathBuf,
    dest_dir: PathBuf,
    config: TargetConfig,
}

pub fn slug_file_name(path: &PathBuf) -> Option<bool> {
    Some(
        path.file_name()?
            .to_str()?
            .contains("{{sire.project_slug}}"),
    )
}

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

impl App {
    pub fn run(&self) -> Result {
        for item in walk_source_dir(&self.src_dir) {
            if item.is_dir() {
                self.copy_dir_to_dest(&item);
            } else if item.is_file() {
                self.copy_file_to_dest(&item);
            }
        }

        Ok(())
    }

    fn copy_dir_to_dest(&self, dir: &PathBuf) -> io::Result<()> {
        let mut dest = PathBuf::new();

        if let Some(_) = slug_file_name(dir) {
            if let Some(parent) = dir.parent() {
                dest.push(&self.dest_dir);
                dest.push(&self.config.project_name);
            }
        } else {
            if let Some(name) = dir.file_name() {
                dest.push(&self.dest_dir);
                dest.push(name);
            }
        }

        println!("{:?}", dest);
        fs::create_dir(dest)?;
        Ok(())
    }

    fn copy_file_to_dest(&self, file: &PathBuf) -> io::Result<()> {
        let mut dest = PathBuf::new();

        if let Some(name) = file.file_name() {
            if name == "config.yml" {
                return Ok(())
            }
            dest.push(&self.dest_dir);
            dest.push(name);
        }

        let template: String = fs::read_to_string(&file)?.parse().unwrap();
        let result = Tera::one_off(
            &template,
            &Context::from_serialize(&self.config).unwrap(),
            true,
        )
        .unwrap();

        fs::write(dest, result).expect("Unable to write file");

        Ok(())
    }
}

impl From<Config> for App {
    fn from(conf: Config) -> Self {
        let target_config = conf.load_target_config();
        Self {
            src_dir: conf.source_dir,
            dest_dir: conf.destination_dir,
            config: target_config,
        }
    }
}
