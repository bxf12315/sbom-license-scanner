use crate::license::LicenseList;
use walker_common::fetcher::{Error, Fetcher, Json};

pub struct SpdxLicense {
    spdx_license_url: String,
}

pub const SPDX_LICENSE_URL: &str = "https://spdx.org/licenses/licenses.json";

#[derive(Debug, thiserror::Error)]
pub enum SpdxLicenseFetcherError {
    #[error("failed to fetcher fedora license data: {0}")]
    FetcherError(#[from] Error),
}

impl SpdxLicense {
    pub fn new(spdx_license_url: &str) -> Self {
        SpdxLicense {
            spdx_license_url: spdx_license_url.to_string(),
        }
    }

    pub async fn load_spdx_license_data(
        &self,
        fetcher: &Fetcher,
    ) -> Result<LicenseList, SpdxLicenseFetcherError> {
        let data = fetcher
            .fetch::<Json<LicenseList>>(self.spdx_license_url.clone())
            .await?
            .into_inner();
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let fetcher = Fetcher::new(Default::default()).await.unwrap();
        let fl = SpdxLicense::new(SPDX_LICENSE_URL)
            .load_spdx_license_data(&fetcher)
            .await
            .unwrap();
        println!("{:?}", fl)
    }
}
