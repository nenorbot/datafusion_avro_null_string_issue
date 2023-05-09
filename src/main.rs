use datafusion::prelude::{AvroReadOptions, SessionContext};

#[tokio::main]
async fn main() {
    let ctx = SessionContext::new();

    ctx.register_avro("t", "alltypes_nulls_plain.avro", AvroReadOptions::default())
        .await
        .unwrap();

    let df = ctx.sql("select * from t").await.unwrap();

    df.show().await.unwrap();
}
