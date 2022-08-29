use clap::Parser;
use reqwest;
use std::collections::HashMap;
use url::Url;
use serde_json;
use serde::{Serialize, Deserialize, Deserializer};
// use regex::Regex;

#[derive(Parser)]
struct CliInputs {
    // Users would input the command like this:
    // `rust-forexcan eur cad`

    /// The currency you want to trade to get to_cur. Case insensitive.
    from_cur: String,
    /// the currency you want to receive by giving from_cur. Case insensitive.
    to_cur: String,
}

#[derive(Deserialize)]
struct BankResponse {
    amount: String,
    // #[serde(deserialize_with = "deserialize_f32")]
    frate: String,
    // #[serde(deserialize_with = "deserialize_f32")]
}

fn deserialize_f32<'de, D, T>(deserializer: D) -> Result<T, D::Error> 
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let string_de = <&str>::deserialize(deserializer)?;
    let return_value = serde_json::from_str(&string_de[..]).map_err(serde::de::Error::custom)?;
    Ok(return_value)
}

struct BankCall {
    url: String,
    params: HashMap<String,String>,
}

impl BankCall {
    fn new(url: String, params: HashMap<String, String>) -> Self {
        Self { url, params }
    }

    fn complete_url(&self) -> Result<Url, url::ParseError> {
        let complete_url = Url::parse_with_params(
            &self.url, 
            &self.params);
        return complete_url;
    }
    
    fn complete_call(&self, in_url: String) -> BankResponse {
        let call_resp = reqwest::blocking::get(in_url)
            .expect("Get Request Failed.");
            // FIXME .json cannot serialize response's string to f32
        // println!("{:?}", call_resp);
        let call: BankResponse = call_resp.json::<BankResponse>().expect("json parser error.");
        return call;
    }

    // fn remove_wrapping_quotes(json_value: &serde_json::Value) -> &str {
    //     let re = Regex::new(r"[0-9.]+").expect(
    //         "Error making regex to filter wrapping quotes. This is a bug!"
    //     );
    //     // println!("{}",re.find(&json_value.as_str().unwrap()).unwrap().as_str());
    //     return re.find(&json_value.as_str().unwrap()).unwrap().as_str();
    // }

    fn execute(&self) {
        // Create the url that we are going to send for our request.
        let complete_url = &self.complete_url().expect(
            "Url could not be parsed. Did you enter your arguments right?"
        );

        let completed_call = &self.complete_call(complete_url.as_str().to_string());
        let rates = completed_call;
        // println!("{}", rates);
        match &rates.frate {
            f32 => println!("RBC's rate for {1} to {2}: {0}", 
                // &BankCall::remove_wrapping_quotes(string),
                f32,
                &self.params["from"], 
                &self.params["to"]),
            _ => println!("Exchange rate not found. Did you enter the currency name right?"),
        }
    }
}

const RBC_RATES_URL: &str = "https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?";
fn call_rbc(from_cur: String, to_cur: String) {
    let rbc_call = BankCall::new(
       RBC_RATES_URL.to_string(),
        HashMap::from([
            ("do".to_string(), "conv".to_string()),
            ("from".to_string(), from_cur.to_string()),
            ("to".to_string(), to_cur.to_string()),
            ("trade".to_string(), "sell".to_string()),
            ("amount".to_string(), "1".to_string()),
        ])
    );    
    
    rbc_call.execute();
}
fn main() {
    // users will submit two arguments telling us what currencies are involved
    let args = CliInputs::parse();
    // println!("Currency conversion: {0} | {1}", args.from_cur, args.to_cur);
    
    call_rbc(args.from_cur, args.to_cur);
}
