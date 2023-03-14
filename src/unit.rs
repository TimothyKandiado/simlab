use crate::error::SimulationError;
use crate::flowsheet::unit_handler::stream_handler::StreamHandler;

pub mod mixer;

pub trait Unit {
    fn simulate(&self, stream_handler: &mut StreamHandler) -> Result<(), SimulationError>;
    fn has_converged(&self) -> bool;
}
