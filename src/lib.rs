use std::collections::HashMap;

use bank_call::BankCall;
use clap::{Parser, ValueEnum};
use anyhow::Result;

mod bank_call;
mod bank_response;

mod rbc;
mod td;

const CURRENCY_ALIAS: HashMap<&str, &str> = HashMap::from([
    ("cad","Canadian Dollar"),
    ("can","Canadian Dollar"),
    ]);
// NEVERMIND:Find out where CIBC makes their rates available from.
// CIBC doesnt carry foreign currency. You have to order ahead. Probs not worth making something for them.
// const CIBC_RATES_URL: &str = "";

#[derive(Parser)]
pub struct CliInputs {
    // Users would input the command like this:
    // `rust-forexcan eur cad`
    /// Params:
    /// from_cur(String) REQUIRED: currency you want to trade to get to_cur. Case insensitive.
    pub from_cur: String,
    /// to_cur(String) REQUIRED: currency you want to receive by giving from_cur. Case insensitive.
    pub to_cur: String,
    // trader(Trader) OPTIONAL: The organization(s) you want to trade with.
    // This is an enum which is validated.
    // Default: All
    // Options (all case insensive)
    // -RBC - Royal Bank of Canada
    // -TD - TD Bank of Canada (NOT AVAILABLE)
    #[clap(default_value = crate::Trader::All)]
    pub trader: Trader,
    // hard_cash(bool) OPTIONAL: Whether you want rates for ordered physical currency.
    // Some orgs, such as TD, offer different rates if you order non-cash vs cash.
    // Setting this to true will simulate ordering physical money.
    // Default: False 
    // #[clap(short = 'c', long = "hard_cash", action)]
    // pub hard_cash: bool,
}

#[derive(Clone, ValueEnum)]
enum Trader {
    All,
    RBC,
    TD,
}

/// Takes a list of user inputs and returns a bank call needing to be made.
/// Errors
/// - If user input is malformed.
pub fn handle_input(inArgs: CliInputs) -> Result<BankCall> {
    // If trader is not a known bank, it is an error.
    let given_trader: Option<&str> = match inArgs.trader {
        All => Some("All"),
        RBC => Some("RBC"),
        TD=> Some("TD"),
        _ => None,
    };
    assert!(given_trader.is_some());
    // Currencies given must be known.

    // Errors handled, call is ready to be returned.

    // User args die after this point, which is fine since we don't need it.
}