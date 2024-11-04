use crate::entity::spdx_license;
use crate::fetcher::license_content_fetcher::{SpdxLicenseContent, SpdxLicenseContentFetcherError};
use crate::fetcher::license_fetcher::{SpdxLicense, SpdxLicenseFetcherError, SPDX_LICENSE_URL};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use spdx_expression::SpdxExpressionError;
use walker_common::fetcher::Fetcher;

#[derive(Debug, thiserror::Error)]
pub enum SpdxLicenserError {
    #[error("failed to fetcher fedora license data: {0}")]
    spdxLicenseFetcherError(#[from] SpdxLicenseFetcherError),
    #[error("failed to fetcher fedora license data: {0}")]
    spdxLicenseContentFetcherError(#[from] SpdxLicenseContentFetcherError),
    #[error("failed to parse SpdxExpression data: {0}")]
    spdxExpressionError(#[from] SpdxExpressionError),
    #[error("init fetcher failed {0}")]
    fetcherInitError(#[from] anyhow::Error),
    #[error("error from sea_orm db {0}")]
    dbError(#[from] sea_orm::DbErr),
}

pub async fn load_spdx_license_data(db: &DatabaseConnection) -> Result<(), SpdxLicenserError> {
    let mut insert_model = Vec::with_capacity(400);
    let fetcher = Fetcher::new(Default::default()).await?;
    let spdx_license = SpdxLicense::new(SPDX_LICENSE_URL)
        .load_spdx_license_data(&fetcher)
        .await?;
    for sl in spdx_license.licenses {
        let spdx_license_content = SpdxLicenseContent::new(sl.detailsUrl.as_str())
            .load_spdx_license_content_data(&fetcher)
            .await;
        match spdx_license_content {
            Ok(sld) => {
                if let Some(comments) = &sld.license_comments {
                    insert_model.push(spdx_license::ActiveModel {
                        id: Default::default(),
                        spdx_id: Set(String::from(&sl.licenseId)),
                        spdx_name: Set(String::from(&sl.name)),
                        detailsUrl: Set(String::from(&sl.detailsUrl)),
                        licenseText: Set(String::from(&sld.license_text)),
                        licenseComments: Set(String::from(comments)),
                        licenseTextHtml: Set(String::from(&sld.license_text_html)),
                    });
                } else {
                    insert_model.push(spdx_license::ActiveModel {
                        id: Default::default(),
                        spdx_id: Set(String::from(&sl.licenseId)),
                        spdx_name: Set(String::from(&sl.name)),
                        detailsUrl: Set(String::from(&sl.detailsUrl)),
                        licenseText: Set(String::from(&sld.license_text)),
                        licenseComments: Set(String::from("")),
                        licenseTextHtml: Set(String::from(&sld.license_text_html)),
                    });
                }
            }
            Err(er) => {
                println!("{:?}", er);
            }
        }
    }

    let _r = spdx_license::Entity::insert_many(insert_model)
        .exec(db)
        .await?;

    Ok(())
}

pub async fn find_spdx_license_by_spdx_id(
    db: &DatabaseConnection,
    spdx_id: &str,
) -> Result<Vec<spdx_license::Model>, SpdxLicenserError> {
    let data = spdx_license::Entity::find()
        .filter(Condition::all().add(spdx_license::Column::SpdxId.eq(spdx_id)))
        .all(db)
        .await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::api::load_spdx_license_data;
    use crate::entity::spdx_license;
    use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

    #[tokio::test]
    async fn it_works() {
        let db = Database::connect("sqlite://../../data/licenseDb.db")
            .await
            .unwrap();
        let schema = Schema::new(DbBackend::Sqlite);

        let stmt = schema
            .create_table_from_entity(spdx_license::Entity)
            .if_not_exists()
            .to_owned();

        let backend = db.get_database_backend();

        db.execute(backend.build(&stmt)).await.unwrap();

        let r = load_spdx_license_data(&db).await;
    }
}
