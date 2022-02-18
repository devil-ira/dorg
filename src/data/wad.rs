use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{Result, bail};
use globwalk;
use serde::de::{self, MapAccess};
use serde::{Deserialize, Deserializer, de::Visitor};

use druid::{Data, Lens};

#[derive(Clone, Data, Lens, Debug)]
pub struct Wad {
    pub name: String,
    pub optional: bool,
}

pub fn find(name: &String) -> Result<PathBuf> {

    let glob = globwalk::glob_builder(format!("wads/**/{}", name))
        .case_insensitive(true);
    for entry in glob.build().unwrap().into_iter().filter_map(|r| r.ok()) {
        return Ok(entry.path().to_owned());
    }
    bail!("Not found.");
}

impl FromStr for Wad {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wad {
            name: s.to_string(),
            optional: false,
        })
    }
}

impl<'de> Deserialize<'de> for Wad {
    fn deserialize<D>(deserializer: D) -> Result<Wad, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WadVisitor)
    }
}

// Allow a Wad to be deserialized from both strings and maps
struct WadVisitor;
impl<'de> Visitor<'de> for WadVisitor {
    type Value = Wad;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Wad::from_str(value).map_err(|e| de::Error::custom(e))
    }

    fn visit_map<M>(self, visitor: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let DeserializeWad { name, optional } =
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))?;
        Ok(Wad {
            name,
            optional,
        })
    }
}

// Used only to avoid infinite recursion inside deserialize
#[derive(Deserialize)]
struct DeserializeWad {
    name: String,
    #[serde(default)]
    optional: bool,
}