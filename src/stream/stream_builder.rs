use std::collections::HashMap;

use super::{material::Material, Stream};

pub struct StreamBuilder {
    temperature: f64,
    pressure: f64,
    materials: HashMap<String, Material>
}

impl StreamBuilder {
    pub fn new() -> StreamBuilder {
        StreamBuilder { temperature: 0.0, pressure: 0.0, materials: HashMap::new() }
    }

    pub fn add_material(mut self, new_material: Material) -> StreamBuilder {
        if let Some(present_material) = self.materials.get_mut(new_material.name.as_str()) {
            present_material.mass += new_material.mass;
        }
        else {
            self.materials.insert(new_material.name.clone(), new_material);
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