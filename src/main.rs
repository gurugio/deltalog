/// Use datafusion to process a SQL statement with parquet files
/// https://github.com/apache/arrow-datafusion/blob/master/datafusion-examples/examples/parquet_sql_multiple_files.rs
///
use datafusion::common::{FileType, GetExt};
use datafusion::datasource::file_format::parquet::ParquetFormat;
use datafusion::datasource::listing::ListingOptions;
use datafusion::error::Result;
use datafusion::prelude::*;
use std::sync::Arc;

/// This example demonstrates executing a simple query against an Arrow data source (a directory
/// with multiple Parquet files) and fetching results
#[tokio::main]
async fn main() -> Result<()> {
    // create local execution context
    let ctx = SessionContext::new();

    // Configure listing options
    // with_enable_pruning will enable partition pruning: What is the partition pruning?
    let file_format = ParquetFormat::default().with_enable_pruning(Some(true));
    let listing_options =
        ListingOptions::new(Arc::new(file_format)).with_file_extension(FileType::PARQUET.get_ext());

    // Register a listing table - this will use all files in the directory as data sources
    // for the query
    // TODO: create testdata directory and put more parquet files
    // TODO: how to spcify some of parquet files? It's not efficient to read all files. It's better to read only necessary files. see deltaquery
    ctx.register_listing_table(
        "logtable", // Is it table name? Or any name for just context?
        &format!("file:///home/gurugio/study/deltalog/"),
        listing_options,
        None,
        None,
    )
    .await
    .unwrap();

    // execute the query
    let df = ctx
        .sql(
            "SELECT * \
        FROM logtable \
        WHERE logtime >= '2023-12-03' AND logtime <= '2023-12-05'",
        )
        .await?;

    // print the results
    df.show().await?;

    Ok(())
}
