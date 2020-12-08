use tokio_postgres::{NoTls, Error};

async fn tx_work(trans: &tokio_postgres::Transaction) {
    trans.query("select * from abc", &[]).await.unwrap();
    trans.commit().await.unwrap();
}


#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (mut client, _connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    let trans = client.transaction().await.unwrap();
    tx_work(&trans).await;

    Ok(())
}