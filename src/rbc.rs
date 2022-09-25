/// This module handles requests being made to the
/// Royal Bank of Canada (RBC).
use crate::bank_call::BankCall;
use std::collections::HashMap;

const RBC_RATES_URL: &str =
    "https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?";

/// Issues request to RBC forex API. Automatically executes it.
/// 
/// # Examples
/// ```
/// let call_response = rbcforex::rbc::call_unit(
///     "CAD".to_string(), "EUR".to_string());
/// ```
/// 
/// # TODO
/// - figure out how to test this function (cant use specific rates)
/// - refactor: don't print response, let caller handle it.
pub fn call_unit(from_cur: String, to_cur: String) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_call() {

        // assert!();
    }
}