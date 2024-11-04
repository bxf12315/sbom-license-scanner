use crate::entity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LicenseCollection(pub(crate) HashMap<String, LicenseInfo>);

#[derive(Serialize, Deserialize, Debug)]
pub struct LicenseInfo {
    pub license: Option<License>,
    pub fedora: Option<Fedora>,
    pub approved: String,
    pub fedora_abbrev: Option<String>,
    pub fedora_name: Option<String>,
    pub spdx_abbrev: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct License {
    pub expression: String,
    pub status: Vec<String>,
    pub url: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fedora {
    #[serde(rename = "legacy-name")]
    pub legacy_name: Option<Vec<String>>,
    #[serde(rename = "legacy-abbreviation")]
    pub legacy_abbreviation: Option<Vec<String>>,
}
