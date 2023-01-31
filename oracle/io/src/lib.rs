#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, TypeInfo};

pub struct OracleMetadata;

impl Metadata for OracleMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Oracle;
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct Oracle {
    pub owner: ActorId,
    pub manager: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConfig {
    pub owner: ActorId,
    pub manager: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Action {
    RequestValue,
    ChangeManager(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    NewValue { value: u128 },
    NewManager(ActorId),
}
