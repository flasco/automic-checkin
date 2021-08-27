mod lib;
use lib::*;
use std::error::Error;

#[derive(Debug, serde::Serialize)]
struct Requestx {
    user: String,
    pwd: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let obj = Requestx {
        user: "ads".to_string(),
        pwd: "dscxv".to_string(),
    };

    let resp = post(
        "https://mcs.snssdk.com/v1/list",
        obj,
    )
    .await?;

    println!("{:?}", resp);

    Ok(())
}
