use crate::entity::fedora_license;
use crate::fetcher::license_fetcher::{FedoraLicense, LicenseFetcherError, FEDORA_LIECENSE_URL};
use sea_orm::ActiveValue::Set;
use sea_orm::Condition;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use spdx_expression::{SpdxExpression, SpdxExpressionError};
use std::fmt::Pointer;
use walker_common::fetcher::Fetcher;
#[derive(Debug, thiserror::Error)]
pub enum FedoraLicenserError {
    #[error("failed to fetcher fedora license data: {0}")]
    licenseFetcherError(#[from] LicenseFetcherError),
    #[error("failed to parse SpdxExpression data: {0}")]
    spdxExpressionError(#[from] SpdxExpressionError),
    #[error("init fetcher failed {0}")]
    fetcherInitError(#[from] anyhow::Error),
    #[error("error from sea_orm db {0}")]
    dbError(#[from] sea_orm::DbErr),
}

pub async fn ingest_fedora_license(db: &DatabaseConnection) -> Result<(), FedoraLicenserError> {
    let mut insert_model = Vec::with_capacity(400);
    let fetcher = Fetcher::new(Default::default()).await?;
    let fl = FedoraLicense::new(fetcher, FEDORA_LIECENSE_URL)
        .load_fedora_license_data()
        .await?;

    for (k, licenseinfo) in &fl.0 {
        let approved = &licenseinfo.approved;

        if let Some(spdx) = &licenseinfo.license {
            if let Some(fedora) = &licenseinfo.fedora {
                if (!spdx.expression.contains("LicenseRef-")) {
                    let spdx_list = SpdxExpression::parse(spdx.expression.as_str())?;
                    for spdx_license in spdx_list.licenses() {
                        if let Some(fedora_license) = &fedora.legacy_abbreviation {
                            for license in fedora_license {
                                insert_model.push(fedora_license::ActiveModel {
                                    id: Default::default(),
                                    fedora_abbrev: Set(String::from(license)),
                                    spdx_abbrev: Set(String::from(&spdx_license.identifier)),
                                    approved: Set(String::from(approved)),
                                });
                            }
                        }
                    }
                } else {
                    if let Some(fedora_license) = &fedora.legacy_abbreviation {
                        for license in fedora_license {
                            insert_model.push(fedora_license::ActiveModel {
                                id: Default::default(),
                                fedora_abbrev: Set(String::from(license)),
                                spdx_abbrev: Set(String::from(&spdx.expression)),
                                approved: Set(String::from(approved)),
                            });
                        }
                    }
                }
            }
        }
        if let Some(spdx) = &licenseinfo.spdx_abbrev {
            if let Some(fedora) = &licenseinfo.fedora_abbrev {
                if (!spdx.contains("LicenseRef-")) {
                    let spdx_list = SpdxExpression::parse(spdx)?;
                    for spdx_license in spdx_list.licenses() {
                        insert_model.push(fedora_license::ActiveModel {
                            id: Default::default(),
                            fedora_abbrev: Set(fedora.to_string()),
                            spdx_abbrev: Set(String::from(&spdx_license.identifier)),
                            approved: Set(String::from(approved)),
                        });
                    }
                } else {
                    insert_model.push(fedora_license::ActiveModel {
                        id: Default::default(),
                        fedora_abbrev: Set(fedora.to_string()),
                        spdx_abbrev: Set(String::from(spdx)),
                        approved: Set(String::from(approved)),
                    });
                }
            }
        }
    }

    let _r = fedora_license::Entity::insert_many(insert_model)
        .exec(db)
        .await?;
    Ok(())
}

pub async fn find_spdx_id_by_fedora_abbrev(
    db: &DatabaseConnection,
    fedora_id: &str,
) -> Result<Vec<fedora_license::Model>, FedoraLicenserError> {
    let data = fedora_license::Entity::find()
        .filter(Condition::all().add(fedora_license::Column::FedoraAbbrev.eq(fedora_id)))
        .all(db)
        .await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::api::ingest_fedora_license;
    use crate::entity::fedora_license;
    use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

    #[tokio::test]
    async fn it_works() {
        let db = Database::connect("sqlite://../../data/licenseDb.db")
            .await
            .unwrap();
        let schema = Schema::new(DbBackend::Sqlite);

        let stmt = schema
            .create_table_from_entity(fedora_license::Entity)
            .if_not_exists()
            .to_owned();

        let backend = db.get_database_backend();

        db.execute(backend.build(&stmt)).await.unwrap();

        let r = ingest_fedora_license(&db).await;
    }
}
