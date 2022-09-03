use clap::Parser;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use url::Url;

#[derive(Parser)]
pub struct CliInputs {
    // Users would input the command like this:
    // `rust-forexcan eur cad`
    /// The currency you want to trade to get to_cur. Case insensitive.
    pub from_cur: String,
    /// the currency you want to receive by giving from_cur. Case insensitive.
    pub to_cur: String,
}

#[derive(Deserialize)]
struct BankResponse {
    // F64 is used because it does not seem to work to deserialize into F32.
    //     I've checked the serde docs and it does not seem to be a f32 method.
    // #[serde(deserialize_with = "deserialize_f64")]
    // amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    frate: f64,
}

fn deserialize_f64<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    // FIXME: This does not translate values to f32. Its because the string
    //  is read in as \"1.26\" or similar. I think the \ is breaking.
    let string_de = <&str>::deserialize(deserializer)?;
    let return_value = serde_json::from_str(string_de).map_err(serde::de::Error::custom)?;
    Ok(return_value)
}

struct BankCall {
    url: String,
    params: HashMap<String, String>,
}

impl BankCall {
    fn new(url: String, params: HashMap<String, String>) -> Self {
        Self { url, params }
    }

    fn complete_url(&self) -> Result<Url, url::ParseError> {
        Url::parse_with_params(&self.url, &self.params)
    }

    fn complete_call(&self, in_url: String) -> BankResponse {
        let call_resp = reqwest::blocking::get(in_url).expect("Get Request Failed.");
        // FIXME .json cannot serialize response's string to f32
        // println!("{:?}", call_resp);
        call_resp
            .json::<BankResponse>()
            .expect("json parser error.")
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
        let complete_url = &self
            .complete_url()
            .expect("Url could not be parsed. Did you enter your arguments right?");

        let completed_call = &self.complete_call(complete_url.as_str().to_string());
        let rates = completed_call;
        // let f64 = &rates.frate; {
        println!(
            "RBC's rate for {1} to {2}: {0}",
            // &BankCall::remove_wrapping_quotes(string),
            &rates.frate,
            &self.params["from"],
            &self.params["to"]
        );
        // _ => println!("Exchange rate not found. Did you enter the currency name right?"),
    }
}

const RBC_RATES_URL: &str =
    "https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?";
pub fn call_rbc(from_cur: String, to_cur: String) {
    let rbc_call = BankCall::new(
        RBC_RATES_URL.to_string(),
        HashMap::from([
            ("do".to_string(), "conv".to_string()),
            ("from".to_string(), from_cur),
            ("to".to_string(), to_cur),
            ("trade".to_string(), "sell".to_string()),
            ("amount".to_string(), "1".to_string()),
        ]),
    );

    rbc_call.execute();
}
