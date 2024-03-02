use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use std::io::Cursor;
use std::collections::HashMap;
use serde_json::to_string;

#[http_component]
async fn handle_csvjson(req: Request) -> anyhow::Result<impl IntoResponse> {
    let (status, body) = match *req.method() {
        Method::Post => {
            let body: Vec<u8> = req.body().to_vec();
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(Cursor::new(body));
        
            let header = rdr.headers()?.clone();
        
            let mut records = Vec::new();
            for result in rdr.records() {
                match result {
                    Ok(record) => {
                        //let record_map: HashMap<_, _> = header.iter().zip(record.iter()).collect();
                        let record_map: HashMap<_, _> = header.iter().zip(record.iter()).map(|(header, value)| (header.to_owned(), value.to_owned())).collect();
                        //let record_json = json!(record_map);
                        //println!("{}", record_json);
                        records.push(record_map);
                    }
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            let json_response = to_string(&records)?;
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json_response.to_string())
                .build())
        }
        Method::Get => {
            Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("Hello, World!".to_string())
                .build())
        }
        _ => {
            Ok(Response::builder()
                .status(405)
                .header("content-type", "text/plain")
                .body("Method Not Allowed".to_string())
                .build())
        }
    };
    Ok(Response::new(status, body))
}