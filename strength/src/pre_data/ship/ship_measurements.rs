use serde::Deserialize;


/// Ship measurements:
/// - length_design_waterline - конструктивная ватерлиния,
/// - number_spatiums - количество теоретических шпаций,
#[derive(Deserialize, Debug)]
pub struct ShipMeasurements {
    length_design_waterline: f64,
    number_spatiums: i64,


}

impl ShipMeasurements {
    pub fn new(length_design_waterline: f64, number_spatiums: i64,) -> Self {
        ShipMeasurements { length_design_waterline, number_spatiums }
    }

    pub fn length_spatium(&self) -> f64 {
        self.length_design_waterline / self.number_spatiums as f64
    }
}