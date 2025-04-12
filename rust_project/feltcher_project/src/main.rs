use std::{fs::File, io::Write, thread, time::Duration};
use serde_json::Value;

trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64) -> Result<(), String>;
    fn id(&self) -> &'static str;
}

struct Bitcoin;
struct Ethereum;
struct SP500;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let data = fetch_crypto_prices()?;
        data.get("bitcoin")
            .and_then(|b| b.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| "Failed to parse Bitcoin price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open("bitcoin_prices.csv")
            .map_err(|e| format!("Failed to open bitcoin_prices.csv: {}", e))?;
        writeln!(file, "{}", price)
            .map_err(|e| format!("Failed to write to bitcoin_prices.csv: {}", e))?;
        Ok(())
    }

    fn id(&self) -> &'static str {
        "Bitcoin"
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let data = fetch_crypto_prices()?;
        data.get("ethereum")
            .and_then(|e| e.get("usd"))
            .and_then(|p| p.as_f64())
            .ok_or_else(|| "Failed to parse Ethereum price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open("ethereum_prices.csv")
            .map_err(|e| format!("Failed to open ethereum_prices.csv: {}", e))?;
        writeln!(file, "{}", price)
            .map_err(|e| format!("Failed to write to ethereum_prices.csv: {}", e))?;
        Ok(())
    }

    fn id(&self) -> &'static str {
        "Ethereum"
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response = ureq::get(url)
            .call()
            .map_err(|e| format!("Failed to fetch S&P 500 data: {}", e))?;
        let json: Value = serde_json::from_reader(response.into_reader())
            .map_err(|e| format!("Failed to parse S&P 500 JSON: {}", e))?;
        let price = json
            .get("chart")
            .and_then(|c| c.get("result"))
            .and_then(|r| r.get(0))
            .and_then(|r| r.get("indicators"))
            .and_then(|i| i.get("quote"))
            .and_then(|q| q.get(0))
            .and_then(|q| q.get("close"))
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.iter().filter_map(|v| v.as_f64()).last())
            .ok_or_else(|| "Failed to parse S&P 500 price".to_string())?;
        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let mut file = File::options()
            .create(true)
            .append(true)
            .open("sp500_prices.csv")
            .map_err(|e| format!("Failed to open sp500_prices.csv: {}", e))?;
        writeln!(file, "{}", price)
            .map_err(|e| format!("Failed to write to sp500_prices.csv: {}", e))?;
        Ok(())
    }

    fn id(&self) -> &'static str {
        "S&P 500"
    }
}

fn fetch_crypto_prices() -> Result<Value, String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
    let response = ureq::get(url)
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(429, _) => "Rate limit exceeded for CoinGecko".to_string(),
            _ => format!("Failed to fetch crypto prices: {}", e),
        })?;
    let json: Value = serde_json::from_reader(response.into_reader())
        .map_err(|e| format!("Failed to parse crypto JSON: {}", e))?;
    Ok(json)
}

fn main() {
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin),
        Box::new(Ethereum),
        Box::new(SP500),
    ];

    println!("Starting price fetcher...");

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price for {}: ${:.2}", asset.id(), price);
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving price for {}: {}", asset.id(), e);
                    }
                }
                Err(e) => eprintln!("Error fetching price for {}: {}", asset.id(), e),
            }
        }
        println!("Waiting 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}