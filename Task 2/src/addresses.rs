use std::fs;

use crate::address;

const JSON_FILE_PATH: &str = "src/addresses.json";

#[derive(Debug)]
pub struct Addresses {
    pub addresses: Vec<address::Address>,
}

#[allow(dead_code)]
impl Addresses {
    /// A factory method to create an Addresses instance from a json file.
    pub fn from_json_file(path: &str) -> Result<Self, String> {
        let data = fs::read_to_string(path)
            .map_err(|err| format!("error importing json file: {:?}", err))?;
        Ok(Self {
            addresses: serde_json::from_str(&data)
                .map_err(|err| format!("error deserializing json string: {:?}", err))?,
        })
    }

    /// The solution to b.
    pub fn pretty_print_addresses(&self) {
        self.addresses.iter().for_each(|addr| println!("{addr}"));
    }

    /// The solution to e.
    pub fn validate_addresses(&self) -> Vec<String> {
        let mut err_strings = Vec::new();
        self.addresses.iter().for_each(|addr| {
            let errs = addr.validate();
            if !errs.is_empty() {
                err_strings.push(format!(
                    "Address for ID: {} is invalid. Validation errors: {:?}",
                    addr.id, errs
                ));
            }
        });
        err_strings
    }

    /// Passes an Addresses instance to a given closure. Used as a helper function for unit tests.
    pub fn with_addresses<F>(run: F)
    where
        F: Fn(Addresses),
    {
        run(Self::from_json_file(JSON_FILE_PATH).expect("error fetching addresses"))
    }
}

#[cfg(test)]
mod tests {
    use super::Addresses;

    #[test]
    fn test_validate_addresses() {
        Addresses::with_addresses(|addrs| {
            assert_eq!(
                addrs.validate_addresses(),
                vec![
                    "Address for ID: 2 is invalid. Validation errors: [\"You must include valid address details (line 1 and/or 2 must be filled in)\"]",
                    "Address for ID: 3 is invalid. Validation errors: [\"You must include a province if your country is ZA\"]",
                ]);
        })
    }
}
