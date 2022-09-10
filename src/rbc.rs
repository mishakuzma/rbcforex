/// This module handles requests being made to the
/// Royal Bank of Canada (RBC).
use crate::bank_call::BankCall;
use std::collections::HashMap;

const RBC_RATES_URL: &str =
    "https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?";

pub fn call(from_cur: String, to_cur: String) {
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
