#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use randomness_oracle_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <RandomnessOracleMetadata as Metadata>::State;

    fn get_owner(state: Self::State) -> ActorId {
        state.owner
    }

    fn get_manager(state: Self::State) -> ActorId {
        state.manager
    }

    fn get_values(state: Self::State) -> Vec<(u128, state::Random)> {
        state
            .values
            .iter()
            .map(|(round, value)| (*round, value.clone()))
            .collect()
    }

    fn get_value(round: u128, state: Self::State) -> state::Random {
        state
            .values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
    }

    fn get_last_round(state: Self::State) -> u128 {
        state.last_round
    }

    fn get_last_random_value(state: Self::State) -> state::RandomSeed {
        state
            .values
            .get(&state.last_round)
            .expect("Unable to find round!")
            .clone()
            .randomness
    }

    fn get_random_value_from_round(round: u128, state: Self::State) -> state::RandomSeed {
        state
            .values
            .get(&round)
            .expect("Unable to find round!")
            .clone()
            .randomness
    }
}
