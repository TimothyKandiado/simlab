use crate::{error::SimulationError, stream::Stream};
use std::collections::hash_map::HashMap;

pub struct StreamHandler {
    pub streams: HashMap<String, Stream>,
}

impl StreamHandler {
    pub fn new() -> StreamHandler {
        return StreamHandler {
            streams: HashMap::new(),
        };
    }

    pub fn verify_streams(&self) -> Result<(), SimulationError> {
        Ok(())
    }

    // probably needs optimisation later
    pub fn get_stream(&mut self, name: &str) -> Option<Stream> {
        let stream_option = self.streams.remove(name);

        if let Some(stream) = stream_option {
            self.streams.insert(name.to_string(), stream.clone());
            return Some(stream);
        }

        None
    }

    pub fn set_stream(&mut self, name: String, stream: Stream) {
        self.streams.insert(name, stream);
    }
}
