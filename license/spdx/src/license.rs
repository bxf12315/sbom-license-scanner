#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct LicenseList {
    pub licenseListVersion: String,
    pub licenses: Vec<License>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct License {
    pub reference: String,
    pub isDeprecatedLicenseId: bool,
    pub detailsUrl: String,
    pub referenceNumber: i32,
    pub name: String,
    pub licenseId: String,
    pub seeAlso: Vec<String>,
    pub isOsiApproved: bool,
    pub isFsfLibre: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct LicenseDetail {
    #[serde(rename = "isDeprecatedLicenseId")]
    pub is_deprecated_license_id: bool,
    #[serde(rename = "isFsfLibre")]
    pub is_fsf_libre: Option<bool>,
    #[serde(rename = "licenseText")]
    pub license_text: String,
    #[serde(rename = "standardLicenseTemplate")]
    pub standard_license_template: String,
    pub name: String,
    #[serde(rename = "licenseComments")]
    pub license_comments: Option<String>,
    #[serde(rename = "licenseId")]
    pub license_id: String,
    #[serde(rename = "crossRef")]
    pub cross_ref: Vec<CrossRef>,
    #[serde(rename = "seeAlso")]
    pub see_also: Vec<String>,
    #[serde(rename = "isOsiApproved")]
    pub is_osi_approved: bool,
    #[serde(rename = "licenseTextHtml")]
    pub license_text_html: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CrossRef {
    #[serde(rename = "match")]
    pub match_: String,
    pub url: String,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "isLive")]
    pub is_live: bool,
    pub timestamp: String,
    #[serde(rename = "isWayBackLink")]
    pub is_way_back_link: bool,
    pub order: i32,
}
