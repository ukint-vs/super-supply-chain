use crate::state;
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    NewManager(ActorId),
    NewRandomValue {
        round: u128,
        value: state::Random,
    },
    LastRoundWithRandomValue {
        round: u128,
        random_value: state::RandomSeed,
    },
}
