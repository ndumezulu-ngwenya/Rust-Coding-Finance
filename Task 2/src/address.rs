use std::fmt::{Debug, Display, Formatter, Result};
use std::string::ToString;

use regex::Regex;
use serde::Deserialize;

const NOT_AVAILABLE: &str = "Not available";

type ValidationError = &'static str;

#[derive(Deserialize, Default, Debug)]
struct CodeAndName {
    #[serde(default)]
    code: String,
    #[serde(default)]
    name: String,
}

type Type = CodeAndName;
type Country = CodeAndName;
type ProvinceOrState = CodeAndName;

impl Country {
    /// Returns true if the country has a non-empty name string.
    fn is_valid_country(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Deserialize, Default, Debug)]
struct LineDetail {
    #[serde(default)]
    line1: String,
    #[serde(default)]
    line2: String,
}

impl Display for LineDetail {
    /// Returns a pretty printing string version of the address line details, or "Not available" if
    /// both lines are empty.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ml1 = self.line1.is_empty();
        let ml2 = self.line2.is_empty();

        match true {
            true if ml1 && ml2 => write!(f, "{}", NOT_AVAILABLE),
            true if ml1 => write!(f, "{}", self.line2),
            true if ml2 => write!(f, "{}", self.line1),
            _ => write!(f, "{}, {}", self.line1, self.line2),
        }
    }
}

impl LineDetail {
    /// returns true if the address line details are valid, i.e. at least one line is not empty.
    fn is_valid_line_detail(&self) -> bool {
        !self.line1.is_empty() || !self.line2.is_empty()
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Address {
    pub id: String,
    #[serde(rename = "type")]
    address_type: Type,
    #[serde(default)]
    #[serde(rename = "addressLineDetail")]
    line_detail: LineDetail,
    #[serde(default)]
    #[serde(rename = "provinceOrState")]
    province_or_state: ProvinceOrState,
    #[serde(default)]
    country: Country,
    #[serde(rename = "cityOrTown")]
    city_or_town: String,
    #[serde(rename = "postalCode")]
    postal_code: String,
    #[serde(default)]
    #[serde(rename = "suburbOrDistrict")]
    suburb_or_district: String,
    #[serde(rename = "lastUpdated")]
    last_updated: String,
}

impl Display for Address {
    /// The solution to a.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            format!("{}: ", self.address_type.name)
                + &format!("{} ", self.line_detail)
                + &format!("- {} ", Self::str_or(&self.city_or_town, NOT_AVAILABLE))
                + &format!(
                    "- {} ",
                    Self::str_or(&self.province_or_state.name, NOT_AVAILABLE)
                )
                + &format!("- {} ", Self::str_or(&self.postal_code, NOT_AVAILABLE))
                + &format!("- {}", Self::str_or(&self.country.name, NOT_AVAILABLE))
        )
    }
}

#[allow(dead_code)]
impl Address {
    /// Checks whether the needed address fields are valid. If a field is not valid a validation
    /// error is added to an error vector.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errs = Vec::new();

        if !self.has_valid_province() {
            errs.push("You must include a province if your country is ZA");
        }
        if !self.country.is_valid_country() {
            errs.push("You must include a country");
        }
        if !self.line_detail.is_valid_line_detail() {
            errs.push("You must include valid address details (line 1 and/or 2 must be filled in)");
        }
        if !Self::is_valid_postal_code(&self.postal_code) {
            errs.push("You must include a valid postal code");
        }

        errs
    }

    /// The solution to d.
    fn is_valid(&self) -> bool {
        self.has_valid_province()
            && self.country.is_valid_country()
            && self.line_detail.is_valid_line_detail()
            && Self::is_valid_postal_code(&self.postal_code)
    }

    /// Returns true if the address has a valid province.
    fn has_valid_province(&self) -> bool {
        match self.country.code.as_str() {
            "ZA" => !self.province_or_state.name.to_string().is_empty(),
            _ => true,
        }
    }

    /// Returns the input string literal if it is not empty otherwise a default value.
    fn str_or<'a>(s: &'a str, default: &'a str) -> &'a str {
        match s {
            "" => default,
            _ => s,
        }
    }

    /// Returns true if the postal code is a numeric value.
    fn is_valid_postal_code(s: &str) -> bool {
        match Regex::new(r"^\d+$") {
            Ok(r) => r.is_match(s),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::addresses::Addresses;

    use super::{Address, ValidationError, NOT_AVAILABLE};

    #[test]
    fn test_is_valid_line_detail() {
        Addresses::with_addresses(|addrs| {
            assert!(addrs.addresses[0].line_detail.is_valid_line_detail());
            assert!(!addrs.addresses[1].line_detail.is_valid_line_detail());
            assert!(addrs.addresses[2].line_detail.is_valid_line_detail());
        })
    }

    #[test]
    fn test_display_for_line_detail() {
        Addresses::with_addresses(|addrs| {
            assert_eq!(
                format!("{}", addrs.addresses[0].line_detail),
                "Address 1, Line 2"
            );
            assert_eq!(format!("{}", addrs.addresses[1].line_detail), NOT_AVAILABLE);
            assert_eq!(format!("{}", addrs.addresses[2].line_detail), "Address 3");
        })
    }

    #[test]
    fn test_is_valid_country() {
        Addresses::with_addresses(|addrs| {
            assert!(addrs.addresses[0].country.is_valid_country());
            assert!(addrs.addresses[1].country.is_valid_country());
            assert!(addrs.addresses[2].country.is_valid_country());
        })
    }

    #[test]
    fn test_display_for_address() {
        Addresses::with_addresses(|addrs| {
            assert_eq!(
                format!("{}", addrs.addresses[0]),
                "Physical Address: Address 1, Line 2 - City 1 - Eastern Cape - 1234 - South Africa"
            );
            assert_eq!(
                format!("{}", addrs.addresses[1]),
                "Postal Address: Not available - City 2 - Not available - 2345 - Lebanon"
            );
            assert_eq!(
                format!("{}", addrs.addresses[2]),
                "Business Address: Address 3 - City 3 - Not available - 3456 - South Africa"
            );
        })
    }

    #[test]
    fn test_validate() {
        Addresses::with_addresses(|addrs| {
            assert_eq!(addrs.addresses[0].validate(), Vec::<ValidationError>::new());
            assert_eq!(
                addrs.addresses[1].validate(),
                vec!["You must include valid address details (line 1 and/or 2 must be filled in)"]
            );
            assert_eq!(
                addrs.addresses[2].validate(),
                vec!["You must include a province if your country is ZA"]
            );
        })
    }

    #[test]
    fn test_is_valid() {
        Addresses::with_addresses(|addrs| {
            assert!(addrs.addresses[0].is_valid());
            assert!(!addrs.addresses[1].is_valid());
            assert!(!addrs.addresses[2].is_valid());
        })
    }

    #[test]
    fn test_has_valid_province() {
        Addresses::with_addresses(|addrs| {
            assert!(addrs.addresses[0].has_valid_province());
            assert!(addrs.addresses[1].has_valid_province());
            assert!(!addrs.addresses[2].has_valid_province());
        })
    }

    #[test]
    fn test_get_pretty_printing_string() {
        assert_eq!(Address::str_or("not_empty", NOT_AVAILABLE), "not_empty");
        assert_eq!(Address::str_or("", NOT_AVAILABLE), NOT_AVAILABLE);
    }

    #[test]
    fn test_is_valid_postal_code() {
        assert!(Address::is_valid_postal_code("1234"));
        assert!(!Address::is_valid_postal_code("abcd"));
        assert!(!Address::is_valid_postal_code("a2c4"));
    }
}
