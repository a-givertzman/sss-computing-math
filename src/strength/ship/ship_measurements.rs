use serde::Deserialize;


/// Ship measurements:
/// - length_design_waterline - длина по конструктивной ватерлинии,
/// - width_design_waterline - ширина по конструктивной ватерлинии,
/// - number_spatiums - количество теоретических шпаций,
#[derive(Deserialize, Debug)]
pub struct ShipMeasurements {
    pub length_design_waterline: f64,
    pub width_design_waterline: f64,
    pub number_spatiums: i64,
}

impl ShipMeasurements {
    pub fn new(length_design_waterline: f64, number_spatiums: i64, width_design_waterline: f64) -> Self {
        ShipMeasurements { length_design_waterline, number_spatiums, width_design_waterline }
    }

    pub fn length_spatium(&self) -> f64 {
        self.length_design_waterline / self.number_spatiums as f64
    }
}