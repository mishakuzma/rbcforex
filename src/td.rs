// This module is to handle making calls to TD bank.
// Under development.

// NOTE TD lets you pick between cash and non cash. 
//  I should add this as a feature for CLI.
// Also note: this one gives you all of the rates in a xml. So it would have
//  to be parsed.
const TD_RATES_URL: &str = 
    "https://tool.td.com/fxcal/api/fxservice/getNonCashFeed";
