use std::collections::HashMap;

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

    pub fn run(&mut self) {
        let result = self.iterate_flowsheet();

        if let Err(error) = result {
            match error {
                SimulationError::StreamNotFound(stream_name) => {println!("Stream not found: {stream_name}");},
                _ => { println!("Serious undefined error occured")}
            }
        }

        else {
            println!("simulation done successfully!")
        }
        
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

    pub fn get_all_streams(&self) -> HashMap<String, Stream> {
        return self.unit_handler.stream_handler.streams.clone();
    }

    fn iterate_flowsheet(&mut self) -> Result<(), SimulationError> {
        for _ in 1..5 {
            let result = self.unit_handler.simulate_units();

            if let Err(error) = result {
                match error {
                    SimulationError::StreamNotFound(_) => {return Err(error)},
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
