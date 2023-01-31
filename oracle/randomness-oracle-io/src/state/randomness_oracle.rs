use super::Random;
use gstd::{prelude::*, ActorId};

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct RandomnessOracle {
    pub owner: ActorId,
    pub values: BTreeMap<u128, Random>,
    pub last_round: u128,
    pub manager: ActorId,
}
