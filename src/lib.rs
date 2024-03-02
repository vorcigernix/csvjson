use spin_sdk::http::{IntoResponse, Request, Response, Method};
use spin_sdk::http_component;
use std::io::Cursor;
use std::collections::HashMap;
use serde_json::to_string;

#[http_component]
async fn handle_csvjson(req: Request) -> anyhow::Result<impl IntoResponse> {
    match req.method() {
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
                        let record_map: HashMap<_, _> = header.iter().zip(record.iter()).map(|(header, value)| (header.to_owned(), value.to_owned())).collect();
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
        _ => {
            const BODY: &str = r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <title>CSV to JSON</title>
                    </head>
                    <body style="text-align:center">
                        <h1>CSV to JSON</h1>
                        <p>This is an API service that converts your CSV into JSON structure. CSV needs to be properly delimited and formatted.&nbsp;</p>
                        <p>Usage:&nbsp;curl --data-binary @****.csv <a href="https://csvjson.fermyon.app">https://csvjson.fermyon.app</a></p>                     
                        <p>Kudos to <a href="https://developer.fermyon.com/">Fermyon</a> for allowing this service to run.</p>
                    </body>
                </html>"#;
            Ok(Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body(BODY.to_string())
            .build())
        }
        
    }

}
