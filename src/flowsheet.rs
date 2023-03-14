use crate::{error::SimulationError, stream::Stream, unit::Unit};

use self::unit_handler::UnitHandler;

pub mod unit_handler;

pub struct Flowsheet {
    unit_handler: UnitHandler,
}

impl Flowsheet {
    pub fn new() -> Flowsheet {
        return Flowsheet {
            unit_handler: UnitHandler::new(),
        };
    }

    pub fn run(self) -> Result<Flowsheet, ()> {
        return Ok(self);
    }

    pub fn add_stream(mut self, name: String, stream: Stream) -> Flowsheet {
        self.unit_handler
            .stream_handler
            .streams
            .insert(name, stream);

        self
    }

    pub fn add_unit(mut self, name: String, unit: Box<dyn Unit>) -> Flowsheet {
        self.unit_handler.units.insert(name, unit);

        self
    }

    pub fn verify_flowsheet(&self) -> Result<(), SimulationError> {
        let stream_verification = self.unit_handler.stream_handler.verify_streams();

        if let Err(error) = stream_verification {
            return Err(error);
        }

        let unit_verification = self.unit_handler.verify_units();

        if let Err(error) = unit_verification {
            return Err(error);
        }

        Ok(())
    }
}
