use clap::Parser;

mod rbc;
mod BankCall;
mod BankResponse;
#[derive(Parser)]
pub struct CliInputs {
    // Users would input the command like this:
    // `rust-forexcan eur cad`
    /// Params:
    /// from_cur(String) REQUIRED: currency you want to trade to get to_cur. Case insensitive.
    pub from_cur: String,
    /// to_cur(String) REQUIRED: currency you want to receive by giving from_cur. Case insensitive.
    pub to_cur: String,
    // trader(String) OPTIONAL: The organization(s) you want to trade with.
    // Default: All
    // Options (all case insensive)
    // -RBC - Royal Bank of Canada
    // -TD - TD Bank of Canada (NOT AVAILABLE)
    // #[clap(default_value_t = String::from("All"))]
    // pub trader: String,
    // hard_cash(bool) OPTIONAL: Whether you want rates for ordered physical currency.
    // Some orgs, such as TD, offer different rates if you order non-cash vs cash.
    // Setting this to true will simulate ordering physical money.
    // Default: False 
    // #[clap(short = 'c', long = "hard_cash", action)]
    // pub hard_cash: bool,
}

// NOTE TD lets you pick between cash and non cash. 
//  I should add this as a feature for CLI.
// Also note: this one gives you all of the rates in a xml. So it would have
//  to be parsed.
const TD_RATES_URL: &str = 
    "https://tool.td.com/fxcal/api/fxservice/getNonCashFeed";
// NEVERMIND:Find out where CIBC makes their rates available from.
// CIBC doesnt carry foreign currency. You have to order ahead. Probs not worth making something for them.
// const CIBC_RATES_URL: &str = "";
