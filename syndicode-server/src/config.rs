use crate::utils::read_env_var;

pub struct Config {
    pub ip_address_header: String,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let ip_address_header = read_env_var("IP_ADDRESS_HEADER")?;

        Ok(Self { ip_address_header })
    }
}
