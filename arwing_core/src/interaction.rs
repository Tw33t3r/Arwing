use serde::de::{Deserialize, Deserializer, Error};

use ssbm_data::character::External;

#[derive(Debug, Clone)]
pub struct Interaction {
    pub action: u16,
    pub from_player: External,
    pub failed_l_cancel: Option<bool>,
    pub within: Option<u32>,
}

#[derive(PartialEq)]
pub enum InteractionResult {
    TimeOut,
    WrongCharacter,
    NonContiguous,
    GameStateMismatch,
    Target,
}

impl Interaction {
    pub fn check_interaction(
        &self,
        frame_state: u16,
        l_cancel_state: Option<u8>,
        remaining: &mut Option<u32>,
        character: External,
    ) -> InteractionResult {
        if let Some(amount) = remaining {
            if amount == &0 {
                return InteractionResult::TimeOut;
            }
            *remaining = Some(*amount - 1);
        }

        if character != self.from_player {
            return InteractionResult::WrongCharacter;
        }

        if let Some(true) = self.failed_l_cancel
            && let Some(1) = l_cancel_state
        {
            return InteractionResult::GameStateMismatch;
        }
        if frame_state == self.action {
            return InteractionResult::Target;
        }
        InteractionResult::NonContiguous
    }
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

        let failed_l_cancel = json.get("failedLCancel").expect("failedLCancel").as_bool();

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
            failed_l_cancel,
            from_player: character,
            within: Some(within),
        })
    }
}

#[derive(Debug, Clone)]
pub enum InteractionCond {
    Single(Interaction),
    All(Vec<InteractionCond>),
    Any(Vec<InteractionCond>),
}

impl InteractionCond {
    pub fn within(&self) -> Option<u32> {
        match self {
            InteractionCond::Single(i) => i.within,
            InteractionCond::All(conds) => conds.iter().filter_map(|c| c.within()).min(),
            InteractionCond::Any(conds) => conds.iter().filter_map(|c| c.within()).max(),
        }
    }

    pub fn matches(
        &self,
        frame_state: u16,
        l_cancel_state: Option<u8>,
        remaining: &mut Option<u32>,
        character: External,
    ) -> InteractionResult {
        match self {
            InteractionCond::Single(cond) => {
                cond.check_interaction(frame_state, l_cancel_state, remaining, character)
            }

            InteractionCond::All(conds) => {
                let is_target = conds.iter().all(|c| {
                    InteractionResult::Target
                        == c.matches(frame_state, l_cancel_state, remaining, character)
                });

                if is_target {
                    return InteractionResult::Target;
                }
                return InteractionResult::NonContiguous;
            }

            InteractionCond::Any(conds) => {
                let is_target = conds.iter().any(|c| {
                    InteractionResult::Target
                        == c.matches(frame_state, l_cancel_state, remaining, character)
                });

                if is_target {
                    return InteractionResult::Target;
                }
                return InteractionResult::NonContiguous;
            }
        }
    }
}

#[derive(Debug)]
pub struct MatchState {
    pub step: usize,
    pub remaining: Option<u32>,
    pub indices: Vec<usize>,
}
