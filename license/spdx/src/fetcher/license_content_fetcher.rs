use crate::license::LicenseDetail;
use walker_common::fetcher::{Error, Fetcher, Json};

pub struct SpdxLicenseContent {
    spdx_license_content_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SpdxLicenseContentFetcherError {
    #[error("failed to fetcher fedora license data: {0}")]
    FetcherError(#[from] Error),
}

impl SpdxLicenseContent {
    pub fn new(spdx_license_content_url: &str) -> Self {
        SpdxLicenseContent {
            spdx_license_content_url: spdx_license_content_url.to_string(),
        }
    }

    pub async fn load_spdx_license_content_data(
        &self,
        fetcher: &Fetcher,
    ) -> Result<LicenseDetail, SpdxLicenseContentFetcherError> {
        let data = fetcher
            .fetch::<Json<LicenseDetail>>(self.spdx_license_content_url.clone())
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
        let fl = SpdxLicenseContent::new("https://spdx.org/licenses/Soundex.json")
            .load_spdx_license_content_data(&fetcher)
            .await
            .unwrap();
        println!("{:?}", fl)
    }
}
