use std::collections::HashMap;
use super::Event;

pub struct EventTable {
    pub mut table: HashMap<u8, Event>,
}
