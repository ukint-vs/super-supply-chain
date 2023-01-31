#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use oracle_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <OracleMetadata as Metadata>::State;

    fn get_owner(state: Self::State) -> ActorId {
        state.owner
    }

    fn get_manager(state: Self::State) -> ActorId {
        state.manager
    }
}
