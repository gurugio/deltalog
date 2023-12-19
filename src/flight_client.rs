use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

use datafusion::arrow::datatypes::Schema;

use arrow_flight::flight_descriptor;
use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::utils::flight_data_to_arrow_batch;
use arrow_flight::{FlightDescriptor, Ticket};
use datafusion::arrow::util::pretty;

/// This example shows how to wrap DataFusion with `FlightService` to support looking up schema information for
/// Parquet files and executing SQL queries against them on a remote server.
/// This example is run along-side the example `flight_server`.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let testdata = datafusion::test_util::parquet_test_data();
    let testdata = format!("file:///home/gurugio/study/deltalog/example.parquet");

    // Create Flight client
    let mut client = FlightServiceClient::connect("http://localhost:50051").await?;

    // Call get_schema to get the schema of a Parquet file
    let request = tonic::Request::new(FlightDescriptor {
        // DescriptorType::Path: a string or list of strings
        r#type: flight_descriptor::DescriptorType::Path as i32,
        // Default::default() generates some default initial values.
        // For DescriptorType::Path type, cmd is not used.
        // cmd is used only when type = CMD
        cmd: Default::default(),
        // list of parquet files
        path: vec![format!("{testdata}")],
    });

    // call get_schema method of FlightServiceClient
    // This schema is used when a consumer needs the Schema of flight stream.
    // Similar to GetFlightInfo??
    // get_schema() returns Response<SchemaResult>
    // into_inner() strips Response.
    let schema_result = client.get_schema(request).await?.into_inner();
    // transfer SchemaResult into Schema
    let schema = Schema::try_from(&schema_result)?;
    println!("after client.get_schema::Schema: {schema:?}");

    // Call do_get to execute a SQL query and receive results
    let request = tonic::Request::new(Ticket {
        //ticket: "SELECT id FROM alltypes_plain".into(),
        ticket: "SELECT * FROM logtable".into(),
    });

    println!("before client.do_get");
    // retrieve a single stream
    let mut stream = client.do_get(request).await?.into_inner();

    // the schema should be the first message returned, else client should error
    let flight_data = stream.message().await?.unwrap();
    // convert FlightData to a stream
    let schema = Arc::new(Schema::try_from(&flight_data)?);
    println!("after client.do_get::Schema: {schema:?}");

    // all the remaining stream messages should be dictionary and record batches
    let mut results = vec![];
    let dictionaries_by_field = HashMap::new();
    while let Some(flight_data) = stream.message().await? {
        let record_batch =
            flight_data_to_arrow_batch(&flight_data, schema.clone(), &dictionaries_by_field)?;
        results.push(record_batch);
    }

    // print the results
    pretty::print_batches(&results)?;

    Ok(())
}
