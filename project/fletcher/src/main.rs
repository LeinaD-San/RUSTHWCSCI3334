use std::{fs::File, io::Write, thread, time ::Duration};
use serde_json::Value;

trait Pricing {
    fn fetch_price(&self, data: &Option<value>) -> f64;
    fn save_to_file(&self, price: f64);
    fn id(&self) -> &'static str;
}

struct Bitcoin;
struct Ethereum;
struct SP500;

impl Pricing for Bitcoin {
    fn fetch_price(&self, data: &Option<value>) -> f64 {
        data.as_ref()
            .and_then (|d| d["bitcoin"]["usd"].as_f64())
            .unwrap_or(0.0)
    }
    fn save_to_file(&self, price: f64) {
        let mut file = File::create("bitcoin.txt").unwrap();
        writeln!(file, "{}", price).unwrap();
    }

    fn id(&self) -> &'static str {
        "bitcoin"
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self, data: &Option<value>) -> f64 {
        data.as_ref()
            .and_then (|d| d["ethereum"]["usd"].as_f64())
            .unwrap_or(0.0)
    }
    fn save_to_file(&self, price: f64) {
        let mut file = File::create("ethereum.txt").unwrap();
        writeln!(file, "{}", price).unwrap();
    }

    fn id(&self) -> &'static str {
        "ethereum"
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self, _: &Option<value>) -> f64 {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url).call();

        match response {
            Ok(resp) => {
                let json: Value = serde_json::from_reader(resp.into_reader()).unwrap();
                let close_prices = &json["chart"]["result"][0]["indicators"]["quote"][0]["close"];
                close_prices
                    .as_array()
                    .and_then(|arr| arr.iter().filter_map(|v| v.as_f64()).last())
                    .unwrap_or(0.0)
            }
            Err(e) => {
                println!("Error fetching SP500 price: {}", e);
                0.0
            }
        }
    }
    fn save_to_file(&self, price: f64) {
        let mut file = File::create("sp500.txt").unwrap();
        writeln!(file, "{}", price).unwrap();
    }

    fn id(&self) -> &'static str {
        "sp500"
    }
}

fn fetch_crypto_prices() -> Option<value> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
    let response = ureq::get(url).call();

    match response {
        Ok(resp) => {
            let json: Value = serde_json::from_reader(resp.into_reader()).unwrap();
            Some(json)
        }
        Err(ureq::Error::Status(429,_)) => {
            println!("");
            None
        }
        Err(e) => {
            println!("Failed to fetch CoinGecko prices {}", e);
            None
        }
    }
}

fn main() {
    let assests: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    loop {
        let crypto_data = fetch_crypto_prices();

        for assest in &assests {
            let price = assest.fetch_price(&crypto_data);
            println!("Fetched price for {}: {}", assest.id(), price);
            assest.save_to_file(price);
        }
        thread::sleep(Duration:: from_secs(10));
    }
}
