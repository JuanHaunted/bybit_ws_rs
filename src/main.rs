use reqwest;
use std::str::FromStr;
use bybit::ws::response::SpotPublicResponse;
use bybit::WebSocketApiClient;



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // We call the coinbase API to check that it's equal to
    // bybit WS response
    let fair_price = {
        let ticker = "SOL-USD";
        let response = reqwest::get(format!(
            "https://api.coinbase.com/v2/prices/{}/spot",
            ticker
        ))
        .await?
        .json::<serde_json::Value>()
        .await?;

        f64::from_str(response["data"]["amount"].as_str().unwrap())?
    };

    // Print fair price fetched by coinbase api
    println!("fair price: ${:.2}", fair_price);


    // Start WS client with ByBit API and fetch last price
    // Print it as long as the connection is open
    let last_price = get_ticker().await?;

    

    Ok(())
}


// Get last price using WebSocketAPIClint from ByBit
async fn get_ticker() -> anyhow::Result<f64> {
    let symbol = "SOLUSDT";
    let mut client = WebSocketApiClient::spot().build();
    let mut last_price = 0.0;

    client.subscribe_ticker(symbol);


    // Note that since the last price variable belongs to the callback, 
    // we cannot exit the function by returning the last price.
    // TODO: Implement sync mechanism with main function to return to main
    // OR If possible transfer the buisiness logic to the callback so the last_price
    // is dynamically updated even then the the rest of the logic is executed in the main function
    let callback = |res: SpotPublicResponse| match res {
        SpotPublicResponse::Ticker(res) => { 
            last_price = f64::from_str(res.data.last_price).unwrap();
            println!("Last price: ${:.2}", last_price)
        },
        _ => println!("Failed to fetch from web socket")
    };

    // Run the callback function
    // Notice the callback logic could implement all other types of connections with
    // the websocket API
    // Here we only show a Spot Ticker Response
    match client.run(callback) {
        Ok(_) => Ok(last_price),
        Err(e) => Err(anyhow::Error::new(e)),
    }

} 