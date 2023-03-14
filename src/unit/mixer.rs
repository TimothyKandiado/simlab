use crate::{stream::{Stream, material::Material}, error::SimulationError, unit::Unit, 
            flowsheet::unit_handler::stream_handler::StreamHandler};

pub struct Mixer {
    input_stream_names: Vec<String>,
    output_stream_name: String,
}

impl Unit for Mixer {
    fn simulate(&self, stream_handler: &mut StreamHandler) -> Result<(), SimulationError> {
        let mut input_streams: Vec<Stream> = Vec::new();

        for input_stream_name in &self.input_stream_names {
            let stream_option = stream_handler.get_stream(input_stream_name.as_str());
            
            if let Some(stream) = stream_option {
                input_streams.push(stream);
            }
            else {
                return Err(SimulationError::StreamNotFound(input_stream_name.clone()));
            }
        }

        let mut output_stream ;
        if let Some(stream) = stream_handler.get_stream(self.output_stream_name.as_str()) {
            output_stream = stream;
        }
        else {
            return Err(SimulationError::StreamNotFound(self.output_stream_name.clone()));
        }

        self.run_mixer(input_streams, &mut output_stream)
    }

    fn has_converged(&self) -> bool {
        return true;
    }
}

impl Mixer {
    fn run_mixer(&self, input_streams: Vec<Stream>, output_stream: &mut Stream) -> Result<(), SimulationError> {
        let mut total_pressure: f64 = 0.0;
        let mut total_temperature: f64 = 0.0;


        let number_of_streams = input_streams.len() as f64;

        for input_stream in input_streams {
            let mut stream_pressure: f64 = 0.0;
            Stream::unwrap_stream_property(&mut stream_pressure, input_stream.pressure)?;

            let mut stream_temperature: f64 = 0.0;
            Stream::unwrap_stream_property(&mut stream_temperature, input_stream.temperature)?;

            total_pressure += stream_pressure;
            total_temperature += stream_temperature;

            let mut stream_materials: Vec<Material> = Vec::new();
            Stream::unwrap_stream_property(&mut stream_materials, input_stream.materials.clone())?;

        }


        output_stream.pressure = Some(total_pressure / number_of_streams);
        output_stream.temperature = Some(total_temperature / number_of_streams);

        Ok(())
    }

}