use crate::state;
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Action {
    SetRandomValue { round: u128, value: state::Random },
    GetLastRoundWithRandomValue,
    UpdateManager(ActorId),
}
