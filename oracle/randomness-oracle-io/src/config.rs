use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConfig {
    pub manager: ActorId,
}
