use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Cab {
    pub name: Option<String>,
    pub address: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub cabs: HashMap<String, Cab>,
}

impl Config {
    pub fn cabs(&self) -> Vec<Cab> {
        let cabs = &self.cabs;
        let cabs = cabs
            .into_iter()
            .map(|(name, cab)| Cab {
                name: Some(name.clone()),
                address: cab.address.clone(),
            })
            .collect();
        cabs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let mut cabs = HashMap::new();
        cabs.insert(
            "Train1".to_string(),
            Cab {
                name: None,
                address: 1,
            },
        );
        let config = Config { cabs };
        assert_eq!(config.cabs()[0].name.as_ref().unwrap(), "Train1");
    }
}
