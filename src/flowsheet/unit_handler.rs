pub mod stream_handler;
use crate::{error::SimulationError, unit::Unit};
use std::collections::hash_map::HashMap;
use stream_handler::StreamHandler;

pub struct UnitHandler {
    pub units: HashMap<String, Box<dyn Unit>>,
    pub stream_handler: StreamHandler,
}

impl UnitHandler {
    pub fn new() -> UnitHandler {
        return UnitHandler {
            units: HashMap::new(),
            stream_handler: StreamHandler::new(),
        };
    }

    pub fn verify_units(&self) -> Result<(), SimulationError> {
        Ok(())
    }
}
