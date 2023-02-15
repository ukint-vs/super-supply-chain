use crate::{state::RandomnessOracle, Action, Event, InitConfig};
use gmeta::{In, InOut, Metadata};

pub struct RandomnessOracleMetadata;

impl Metadata for RandomnessOracleMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = RandomnessOracle;
}
