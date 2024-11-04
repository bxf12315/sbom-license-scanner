// // use crate::fedora::api::{add, ingest_fedora_license};
// use crate::entity::fedora_license;
// use sbom_license_scanner_fedora::api::ingest_fedora_license;
use fedora::api::ingest_fedora_license;
use fedora::entity::fedora_license;
use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let db = Database::connect("sqlite://data/licenseDb.db")
        .await
        .unwrap();
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt = schema
        .create_table_from_entity(fedora_license::Entity)
        .if_not_exists()
        .to_owned();

    let backend = db.get_database_backend();

    let _ = db.execute(backend.build(&stmt)).await.unwrap();

    let _r = ingest_fedora_license(&db).await.unwrap();
}
