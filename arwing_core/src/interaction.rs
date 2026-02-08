use serde::de::{Deserialize, Deserializer, Error};

use ssbm_data::character::External;

#[derive(Debug)]
pub struct Interaction {
    pub action: u16,
    pub from_player: External,
    pub within: Option<u32>,
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: serde_json::value::Value = serde_json::value::Value::deserialize(deserializer)?;
        let state_id: u16 = match json.get("action").expect("action").as_u64() {
            Some(number) => match u16::try_from(number) {
                Ok(u16_number) => u16_number,
                Err(_) => return Err(Error::custom("state_id contained a value out of bounds")),
            },
            None => {
                return Err(Error::custom(
                    "state_id deserializer recieved a non-numeric type",
                ));
            }
        };

        let character_id: u8 = json
            .get("fromPlayer")
            .ok_or_else(|| Error::custom("From_player not present in json blob"))?
            .as_u64()
            .ok_or_else(|| Error::custom("From_player deserializer recieved a non-numeric type"))?
            .try_into()
            .map_err(|_| Error::custom("From_player contained a value out of bounds"))?;

        let character = External::try_from(character_id)
            .map_err(|_| Error::custom("From_player contained a value out of bounds"))?;

        let within = match json.get("within").expect("within").as_u64() {
            Some(number) => match u32::try_from(number) {
                Ok(u32_number) => u32_number,
                Err(_) => return Err(Error::custom("Within contained a value out of bounds")),
            },
            None => {
                return Err(Error::custom(
                    "Within deserializer recieved a non-numeric type",
                ));
            }
        };

        Ok(Interaction {
            action: state_id,
            from_player: character,
            within: Some(within),
        })
    }
}
