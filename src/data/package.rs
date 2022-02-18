use druid::{Data, Lens, im::Vector};
use serde::Deserialize;

use crate::data::wad::Wad;



#[derive(Clone, Data, Lens, Debug)]
pub struct Package {
    pub name: String,
    pub iwad: String,
    pub wads: Vector<Wad>,
    pub hide_if_dependencies_are_missing: bool,
}

impl Package {
    pub fn transform(&self) -> SerializePackage {
        SerializePackage {
            name: self.name.clone(),
            iwad: self.iwad.clone(),
            wads: self.wads.iter().cloned().collect(),
            hide_if_dependencies_are_missing: self.hide_if_dependencies_are_missing,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct SerializePackage {
    name: String,
    iwad: String,
    #[serde(default)]
    wads: Vec<Wad>,
    #[serde(default)]
    hide_if_dependencies_are_missing: bool,
}

impl SerializePackage {
    pub fn transform(&self) -> Package {
        Package {
            name: self.name.clone(),
            iwad: self.iwad.clone(),
            wads: self.wads.clone().into(),
            hide_if_dependencies_are_missing: self.hide_if_dependencies_are_missing,
        }
    }
}