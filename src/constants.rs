use std::path::PathBuf;

use druid::Selector;
use lazy_static::lazy_static;

use crate::{data::package::Package};

lazy_static! {
    pub static ref DOWNLOAD_DIR: PathBuf = PathBuf::from("downloads");
    pub static ref EXTRACT_DIR: PathBuf = PathBuf::from("downloads/extracted");
    pub static ref WADS_DIR: PathBuf = PathBuf::from("wads");
    pub static ref IWADS_DIR: PathBuf = PathBuf::from("wads/iwads");
    pub static ref SAVES_DIR: PathBuf = PathBuf::from("saves");
}

pub const START_PACKAGE: Selector<Package> = Selector::new("start-package");