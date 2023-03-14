use self::material::Material;

pub mod material;
pub mod stream_builder;

use crate::error::SimulationError;

#[derive(Debug, Clone)]
pub struct Stream {
    pub temperature: Option<f64>,
    pub pressure: Option<f64>,
    pub materials: Option<Vec<Material>>,

    is_input: bool,
    is_specified: bool,
}

impl Stream {
    pub fn input_stream(temperature: f64, pressure: f64, materials: Vec<Material>) -> Stream {
        return Stream {
            temperature: Some(temperature),
            pressure: Some(pressure),
            materials: Some(materials),
            is_input: true,
            is_specified: true
        };
    }

    pub fn output_stream() -> Stream {
        return Stream {
            temperature: None,
            pressure: None,
            materials: None,
            is_input: false,
            is_specified: false
        };
    }

    pub fn is_input_stream(&self) -> bool {
        self.is_input
    }

    pub fn set_properties(&mut self, pressure: f64, temperature: f64, materials: Vec<Material>) {
        self.pressure = Some(pressure);
        self.temperature = Some(temperature);
        self.materials = Some(materials);
        self.is_specified = true;
    }

    pub fn is_stream_specified(&self) -> bool {
        return self.is_input || self.is_specified;
    }

    pub fn unwrap_stream_property<T>(output: &mut T, wrapped_value: Option<T>) -> Result<(), SimulationError>{
        if let Some(value) = wrapped_value {
            *output = value;
            return Ok(());
        }
        else {
            return Err(SimulationError::IncompleteInputs);
        }
    }
}
