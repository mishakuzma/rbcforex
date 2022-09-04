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

    /// Complete call takes a String and returns a completed BankResponse.
    /// Inputs
    /// 0: &self
    /// 1: A URL with the queries attached.
    /// Panics
    /// - When the get request cannot be completed
    ///     Either the connection is off, or the url was malformed.
    /// - When the JSON could not be parsed.
    ///     Likely because unexpected fields or data types are present.
    ///     If you find one, it is likely an error to be reported.
    fn complete_call(&self, in_url: String) -> BankResponse {
        let call_resp = reqwest::blocking::get(in_url).expect("Get Request Failed.");
        // println!("{:?}", call_resp);
        call_resp
            .json::<BankResponse>()
            .expect("json parser error.")
    }

    // Legacy function for removing wrapping quotes on string.
    // Not needed since serde deserializer, but it's here in case.
    // fn remove_wrapping_quotes(json_value: &serde_json::Value) -> &str {
    //     let re = Regex::new(r"[0-9.]+").expect(
    //         "Error making regex to filter wrapping quotes. This is a bug!"
    //     );
    //     // println!("{}",re.find(&json_value.as_str().unwrap()).unwrap().as_str());
    //     return re.find(&json_value.as_str().unwrap()).unwrap().as_str();
    // }

    /// Returns a 3 item tuple, where
    /// 0: The unit rate of currency (f64)
    /// 1: The currency that was given (&String)
    /// 2: The currency that was received (&String)
    /// The currencies exchanged are references to the bankcall object fields.
    /// Panics
    /// When the url cannot be parsed, likely because the arguments are wrong.
    fn execute(&self) -> (f64, &String, &String) {
        // Create the url that we are going to send for our request.
        let complete_url = &self
            .complete_url()
            .expect("Url could not be parsed. Did you enter your arguments right?");

        // call the bank for current rates
        let completed_call = &self.complete_call(complete_url.as_str().to_string());

        // TODO: Remove this print line and let it be handled
        println!(
            "RBC's rate for {1} to {2}: {0}",
            &completed_call.frate, &self.params["from"], &self.params["to"]
        );

        // Current rates are returned as owned because they are unique per call.
        // From and To are always available in the call, so refs are fine.
        (
            completed_call.frate,
            &self.params["from"],
            &self.params["to"],
        )
    }
}

const RBC_RATES_URL: &str =
    "https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?";
// NOTE TD lets you pick between cash and non cash. 
//  I should add this as a feature for CLI.
// Also note: this one gives you all of the rates in a xml. So it would have
//  to be parsed.
const TD_RATES_URL: &str = 
    "https://tool.td.com/fxcal/api/fxservice/getNonCashFeed";
// TODO Find out where CIBC makes their rates available from.
// const CIBC_RATES_URL: &str = "";
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
