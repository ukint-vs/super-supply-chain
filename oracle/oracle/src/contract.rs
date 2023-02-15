use gstd::{async_main, debug, msg, prelude::*, ActorId};
use oracle_io::{Action, Event, InitConfig, Oracle};

#[async_trait::async_trait]
pub trait OracleHandler {
    async fn request_value(&mut self);
    fn change_manager(&mut self, new_manager: ActorId);
    fn assert_only_owner(&self);
}

#[async_trait::async_trait]
impl OracleHandler for Oracle {
    async fn request_value(&mut self) {
        debug!("Before sending message to manager");
        let value = msg::send_for_reply_as(self.manager, 0i32, 0)
            .expect("Can't send message for update value")
            .await
            .expect("Can't obtain updated value");
        debug!("After sending message to manager");
        msg::reply(Event::NewValue { value }, 0).expect("Unable to reply!");
    }

    fn change_manager(&mut self, new_manager: ActorId) {
        self.assert_only_owner();

        self.manager = new_manager;

        msg::reply(Event::NewManager(new_manager), 0).expect("Unable to reply!");
    }

    fn assert_only_owner(&self) {
        if msg::source() != self.owner {
            panic!("Only owner allowed to call this function!");
        }
    }
}

static mut ORACLE: Option<Oracle> = None;

#[async_main]
async fn main() {
    let action: Action = msg::load().expect("Unable to decode Action.");
    let oracle: &mut Oracle = unsafe { ORACLE.get_or_insert(Oracle::default()) };

    match action {
        Action::RequestValue => oracle.request_value().await,
        Action::ChangeManager(new_manager) => oracle.change_manager(new_manager),
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig.");
    let oracle = Oracle {
        owner: config.owner,
        manager: config.manager,
    };

    ORACLE = Some(oracle);
}

#[no_mangle]
extern "C" fn metahash() {
    msg::reply(include!("../.metahash"), 0)
        .expect("Failed to encode or reply with `[u8; 32]` from `metahash()`.");
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(
        unsafe { ORACLE.clone().expect("Uninitialized oracle state.") },
        0,
    )
    .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`.");
}
