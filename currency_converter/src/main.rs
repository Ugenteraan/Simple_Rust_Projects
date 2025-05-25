use reqwest;
use serde::Deserialize;
use std::error::Error;



#[derive(Deserialize)]
struct ApiResponse {
    data: std::collections::HashMap<String, f64>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://api.freecurrencyapi.com/v1/latest?apikey=fca_live_J4yzkPZVDTbe0gRF4MoYpNmLpFvaEq1JMQjhJZGC&currencies=AUD";

    let response: ApiResponse = reqwest::get(url).await?.json().await?;


    if let Some(rate) = response.data.get("AUD") {
        println!("Exchange rate (USD -> MYR): {:.2}", rate);
    } else {
        println!("Failed to fetch the exchange rate.");
    }

    Ok(())
}
