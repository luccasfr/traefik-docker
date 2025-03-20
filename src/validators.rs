use core::error::Error;
use inquire::validator::Validation;
use regex::Regex;

const DOMAIN_RE: &str = r"^(?:(?:[a-zA-Z0-9-]{1,63}\.){1,}[a-zA-Z]{2,63})$";
const HOSTNAME_RE: &str = r"^([A-Za-z-]+[A-Za-z-0-9]*):([0-9]{1,5})$";
const IP_RE: &str = r"^([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}):([0-9]{1,5})$";
const DOMAIN_WITH_PORT_RE: &str = r"^(?:(?:[a-zA-Z0-9-]{1,63}\.){1,}[a-zA-Z]{2,63}):([0-9]{1,5})$";

pub fn alphanumeric_validator(val: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    if val
        .chars()
        .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
    {
        return Ok(Validation::Invalid(
            "Only alphanumeric characters and underscores are allowed".into(),
        ));
    }
    Ok(Validation::Valid)
}

pub fn domain_validator(val: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let re = Regex::new(DOMAIN_RE).unwrap();
    for address in val.split(';') {
        let address = address.trim();
        if !re.is_match(address) {
            return Ok(Validation::Invalid(
                format!("Invalid domain: {}", address).into(),
            ));
        }
    }
    Ok(Validation::Valid)
}

pub fn validate_address(val: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    let ip_re = Regex::new(IP_RE).unwrap();
    let hostname_re = Regex::new(HOSTNAME_RE).unwrap();
    let domain_re = Regex::new(DOMAIN_WITH_PORT_RE).unwrap();
    for address in val.split(';') {
        let address = address.trim();
        if !ip_re.is_match(address) && !hostname_re.is_match(address) && !domain_re.is_match(address) {
            return Ok(Validation::Invalid(
                format!("Invalid address: {}", address).into(),
            ));
        }
    }
    Ok(Validation::Valid)
}
