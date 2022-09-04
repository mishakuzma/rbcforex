/// This module is to handle responses from the organizations we are sending to.

use serde::{Deserialize, Deserializer};

/// BankResponse is a struct for handling a completed BankCall. After the
/// data from a BankCall is retrieved, BankCall should be dropped for that run
/// so that no more requests can occur that may confound an individual run.
#[derive(Deserialize)]
pub struct BankResponse {
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
    //  is read in as \"1.26\" or similar. F64 works though. We love floats v doubles. 
    let string_de = <&str>::deserialize(deserializer)?;
    let return_value = serde_json::from_str(string_de).map_err(serde::de::Error::custom)?;
    Ok(return_value)
}

impl BankResponse {
    /// get_frate is a getter for the frate for a completed bank call.
    pub fn get_frate(self: &Self) -> f64 {
       self.frate 
    }
}