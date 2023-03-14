#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub mass: f64,
}

impl Material {
    pub fn build(name: String, mass: f64) -> Material {
        return Material { name, mass };
    }

    pub fn moles2mass(_name: String, moles: f64) -> f64 {
        return moles;
    }

    pub fn get_properties(&self) -> (String, f64) {
        return (self.name.clone(), self.mass);
    }
}
