use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::io::Cursor;
use std::collections::HashMap;
use serde_json::json;

#[http_component]
async fn handle_csvjson(req: Request) -> anyhow::Result<impl IntoResponse> {
    let body: Vec<u8> = req.body().to_vec();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(body));

    let header = rdr.headers()?.clone();

    for result in rdr.records() {
        match result {
            Ok(record) => {
                let record_map: HashMap<_, _> = header.iter().zip(record.iter()).collect();
                let record_json = json!(record_map);
                println!("{}", record_json);
            }
            Err(err) => println!("Error: {:?}", err),
        }
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("Hello, Fermyon")
        .build())
}
