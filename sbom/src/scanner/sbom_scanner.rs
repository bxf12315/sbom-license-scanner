use crate::model::sbom_license::{PackageLicense, SbomLicense, SbomPackage};
use sbom_license_scanner_fedora::api::{find_spdx_id_by_fedora_abbrev, FedoraLicenserError};
use sbom_license_scanner_fedora::entity::fedora_license::Model;
use sbom_license_scanner_spdx::api::{find_spdx_license_by_spdx_id, SpdxLicenserError};
use sbom_license_scanner_spdx::fetcher::license_fetcher::SpdxLicenseFetcherError;
use sbom_walker::Sbom;
use sea_orm::{ColIdx, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use spdx_rs::models::{OtherLicensingInformationDetected, SimpleExpression, SPDX};

pub struct SbomScanner {
    sbom: Sbom,
    db: DatabaseConnection,
}

#[derive(Debug, thiserror::Error)]
pub enum SbomScannerError {
    #[error("failed to fetcher fedora license data: {0}")]
    spdxLicenserError(#[from] SpdxLicenserError),

    #[error("failed to fetcher fedora license data: {0}")]
    fedoraLicenserError(#[from] FedoraLicenserError),
}

impl SbomScanner {
    pub fn new(sbom: Sbom, db: DatabaseConnection) -> Self {
        SbomScanner { sbom: sbom, db: db }
    }

    pub async fn scanner(&self) -> Result<SbomLicense, SbomScannerError> {
        match &self.sbom {
            Sbom::Spdx(spdx_bom) => {
                let (sbom_name, all_packages) = self.handle_spdx_sbom(&spdx_bom).await?;
                let license_result = SbomLicense {
                    sbom_name: sbom_name.to_string(),
                    packages: all_packages,
                };

                Ok(license_result)
            }
            Sbom::CycloneDx(cyclonedx_bom) => Ok(SbomLicense {
                sbom_name: "".to_string(),
                packages: vec![],
            }),
        }
    }

    async fn handle_spdx_sbom(
        &self,
        spdx_bom: &&SPDX,
    ) -> Result<(&String, Vec<SbomPackage>), SbomScannerError> {
        let is_red_hat = &spdx_bom
            .document_creation_information
            .creation_info
            .creators
            .iter()
            .any(|c| c.contains("Red Hat"));

        let sbom_name = &spdx_bom.document_creation_information.document_name;
        let mut all_packages = Vec::new();

        for pi in spdx_bom.package_information {
            let package_name = pi.package_name;
            let package_version = pi.package_version;
            let package_url = pi
                .external_reference
                .iter()
                .find(|r| r.reference_type == "purl")
                .map(|r| r.reference_locator.as_str())
                .unwrap_or("");

            let package_supplier = pi.package_supplier;

            let mut spdx_ids = Vec::new();
            if let Some(license) = pi.declared_license {
                for l in license.licenses() {
                    if l.license_ref {
                        let license_ref = &spdx_bom
                            .other_licensing_information_detected
                            .iter()
                            .filter(|extraced_license| {
                                extraced_license
                                    .license_identifier
                                    .contains(l.identifier.as_str())
                            })
                            .collect::<Vec<OtherLicensingInformationDetected>>()
                            .first();

                        if let Some(license_info) = license_ref {
                            spdx_ids.push(PackageLicense {
                                license_id: license_info.license_identifier.to_string(),
                                name: license_info.license_name.to_string(),
                                license_text: license_info.extracted_text.to_string(),
                                license_text_html: "".to_string(),
                            })
                        }
                    } else {
                        if is_red_hat {
                            self.search_spdx_by_fedora_abbrev(&mut spdx_ids, &l).await?;
                        } else {
                            self.search_spdx_licenses_by_id(
                                &mut spdx_ids,
                                String::from(&l.identifier),
                            )
                            .await?;
                        }
                    }
                }
            }

            let result = SbomPackage {
                name: package_name,
                version: package_version,
                purl: package_url.to_string(),
                supplier: package_supplier,
                licenses: spdx_ids,
            };

            all_packages.push(result);
        }
        Ok((sbom_name, all_packages))
    }

    async fn search_spdx_by_fedora_abbrev(
        &self,
        mut spdx_ids: &mut Vec<PackageLicense>,
        l: &&SimpleExpression,
    ) -> Result<(), SbomScannerError> {
        let fedora_license =
            find_spdx_id_by_fedora_abbrev(&self.db, &l.identifier.as_str()).await?;
        for fm in fedora_license {
            self.search_spdx_licenses_by_id(&mut spdx_ids, fm.spdx_abbrev)
                .await?;
        }
        Ok(())
    }

    async fn search_spdx_licenses_by_id(
        &self,
        spdx_ids: &mut Vec<PackageLicense>,
        spdx_abbrev: String,
    ) -> Result<(), SbomScannerError> {
        let spdx = find_spdx_license_by_spdx_id(&self.db, spdx_abbrev.as_str()).await?;
        for s in spdx {
            spdx_ids.push(PackageLicense {
                license_id: s.spdx_id,
                name: s.spdx_name,
                license_text: s.licenseText,
                license_text_html: s.licenseTextHtml,
            });
        }
        Ok(())
    }
}
