use super::{resources::*};

pub struct StateChangeEvent(pub AppState);
pub struct StatePushEvent(pub AppState);
pub struct StatePopEvent;