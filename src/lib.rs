use sqlx::PgPool;
use sync_wrapper::SyncWrapper;

mod router;
use router::router;

mod database;

#[shuttle_service::main]
async fn axum(#[shuttle_shared_db::Postgres] dbconn: PgPool) -> shuttle_service::ShuttleAxum {
    // sqlx::migrate!().run(&postgres).await.expect("Had an error running migrations");

    let router = router(dbconn);
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
