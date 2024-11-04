use crate::license::LicenseCollection;
use walker_common::fetcher::{Error, Fetcher};

pub struct FedoraLicense {
    fetcher: Fetcher,
    fedora_license_url: String,
}

pub const FEDORA_LIECENSE_URL: &str ="https://gitlab.com/fedora/legal/fedora-license-data/-/jobs/artifacts/main/raw/fedora-licenses.json?job=json";

#[derive(Debug, thiserror::Error)]
pub enum LicenseFetcherError {
    #[error("failed to fetcher fedora license data: {0}")]
    FetcherError(#[from] Error),
    #[error("failed to fetcher fedora license data: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl FedoraLicense {
    pub fn new(fetcher: Fetcher, fedora_license_url: &str) -> Self {
        FedoraLicense {
            fetcher,
            fedora_license_url: fedora_license_url.to_string(),
        }
    }

    pub async fn load_fedora_license_data(&self) -> Result<LicenseCollection, LicenseFetcherError> {
        let data = self
            .fetcher
            .fetch::<String>(self.fedora_license_url.clone())
            .await?;
        let jb: LicenseCollection = serde_json::from_str(&data).unwrap();
        Ok(jb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let fetcher = Fetcher::new(Default::default()).await.unwrap();
        let fl = FedoraLicense::new(fetcher, FEDORA_LIECENSE_URL)
            .await
            .load_fedora_license_data()
            .await
            .unwrap();
        println!("{:?}", fl)
    }
}
