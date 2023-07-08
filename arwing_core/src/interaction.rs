use serde::de::{Deserialize, Deserializer, Error};

use peppi::model::enums::{action_state::State, character::Internal};

#[derive(Debug)]
pub struct Interaction {
    pub action: State,
    pub from_player: Internal,
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
                ))
            }
        };

        let from_player_id: u8 = match json.get("from_player").expect("from_player").as_u64() {
            Some(number) => match u8::try_from(number) {
                Ok(u8_number) => u8_number,
                Err(_) => return Err(Error::custom("From_player contained a value out of bounds")),
            },
            None => {
                return Err(Error::custom(
                    "From_player deserializer recieved a non-numeric type",
                ))
            }
        };
        let character = Internal(from_player_id);

        let within = match json.get("within").expect("within").as_u64() {
            Some(number) => match u32::try_from(number) {
                Ok(u32_number) => u32_number,
                Err(_) => return Err(Error::custom("Within contained a value out of bounds")),
            },
            None => {
                return Err(Error::custom(
                    "Within deserializer recieved a non-numeric type",
                ))
            }
        };

        Ok(Interaction {
            action: State::from(state_id, character),
            from_player: character,
            within: Some(within),
        })
    }
}
