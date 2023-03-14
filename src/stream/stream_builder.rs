use std::collections::HashMap;

use super::{material::Material, Stream};

pub struct StreamBuilder {
    temperature: f64,
    pressure: f64,
    materials: HashMap<String, Material>
}

impl StreamBuilder {
    pub fn new(pressure: f64, temperature: f64) -> StreamBuilder {
        StreamBuilder { temperature, pressure, materials: HashMap::new() }
    }

    pub fn add_material(mut self, material: Material) -> StreamBuilder {
        if let Some(material) = self.materials.get_mut(material.name.as_str()) {
            material.mass += material.mass;
        }
        else {
            self.materials.insert(material.name.clone(), material);
        }

        self
    }

    pub fn set_temperature(mut self, value: f64) -> StreamBuilder {
        self.temperature = value;
        self
    }

    pub fn set_pressure(mut self, value: f64) -> StreamBuilder {
        self.pressure = value;
        self
    }

    pub fn build(self) -> Stream{
        let mut materials: Vec<Material> = Vec::new();
        let temperature = Some(self.temperature);
        let pressure = Some(self.pressure);

        for (_, material) in self.materials {
            materials.push(material);
        }

        let materials = Some(materials);

        return Stream {
            pressure,
            temperature,
            materials,
            is_input :false,
            is_specified : true,
        }
    }
}