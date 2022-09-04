/// This module handles how we make calls to the various banks for rates.
/// Note: This does not need to be exclusively for banks, but it was for banks
/// when this crate was made originally.
use std::collections::HashMap;
use url::Url;
use crate::BankResponse::BankResponse;
pub struct BankCall {
    url: String,
    params: HashMap<String, String>,
}

impl BankCall {
    pub fn new(url: String, params: HashMap<String, String>) -> Self {
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
    pub fn execute(&self) -> (f64, &String, &String) {
        // Create the url that we are going to send for our request.
        let complete_url = &self
            .complete_url()
            .expect("Url could not be parsed. Did you enter your arguments right?");

        // call the bank for current rates
        let completed_call = &self.complete_call(complete_url.as_str().to_string());

        // TODO: Remove this print line and let it be handled
        println!(
            "RBC's rate for {1} to {2}: {0}",
            completed_call.get_frate(), &self.params["from"], &self.params["to"]
        );

        // Current rates are returned as owned because they are unique per call.
        // From and To are always available in the call, so refs are fine.
        // TODO This makes sense for RBC, but not for other banks.
        // TODO It would be more readable as a hashmap tbh
        (
            completed_call.get_frate(),
            &self.params["from"],
            &self.params["to"],
        )
    }
}