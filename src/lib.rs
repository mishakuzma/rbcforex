use std::collections::HashMap;

use bank_call::BankCall;
use clap::{Parser, ValueEnum, arg_enum};
use anyhow::{Result, Context, anyhow};

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
    #[clap(default_value = "All")]
    pub trader: Trader,
    // hard_cash(bool) OPTIONAL: Whether you want rates for ordered physical currency.
    // Some orgs, such as TD, offer different rates if you order non-cash vs cash.
    // Setting this to true will simulate ordering physical money.
    // Default: False 
    // #[clap(short = 'c', long = "hard_cash", action)]
    // pub hard_cash: bool,
}

arg_enum!{
    #[derive(Clone, ValueEnum)]
    enum Trader {
        All,
        RBC,
        TD,
    }
}

trait ConvertEnumToUrl {
    fn convert_to_url(&self) -> Result<String>;
}

impl ConvertEnumToUrl for Trader {
    fn convert_to_url(&self) -> Result<String> {
        // TODO Trader should not handle conversion to url, because it will
        //  be expected to handle the All variant
       match self {
           Trader::All => Ok("https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?".to_string()),
           Trader::RBC => Ok("https://online.royalbank.com/cgi-bin/tools/foreign-exchange-calculator/rates.cgi?".to_string()),
           Trader::TD => Err(anyhow!("TD not implemented yet."))
       } 
    }
}
/// Takes an inputted trader and returns a string of the trader
/// if the trader is valid.
/// 
/// # Examples
/// ```
/// let result = rust_forexcan::check_trader(Trader::RBC);
/// assert(result.is_ok());
/// ```
/// 
fn check_trader(in_given_trader: Trader) -> anyhow::Result<Trader> {
    // If trader is not a known bank, it is an error.
    match in_given_trader {
        All => Ok(All),
        RBC => Ok(RBC),
        TD=> Ok(TD),
        _ => Err(anyhow!("Unknown trader")),
    }
}

/// Takes a list of user inputs and returns a bank call needing to be made.
/// Errors
/// - If user input is malformed.
pub fn handle_input(inArgs: CliInputs) -> Result<BankCall> {
    // Compose the bankcall, checking for validating errors along the way
    let structed_bankcall = BankCall::new(
        check_trader(inArgs.trader)?
            .convert_to_url()?,
        HashMap::from([
            ("do".to_owned(), "conv".to_owned())
            ]));
    Ok(structed_bankcall)
    // User args die after this point, which is fine since we don't need it now.
}