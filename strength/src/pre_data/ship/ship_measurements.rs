use serde::Deserialize;


/// Ship measurements:
/// - length_design_waterline - длина по конструктивной ватерлинии,
/// - width_design_waterline - ширина по конструктивной ватерлинии,
/// - number_spatiums - количество теоретических шпаций,
/// - draft - осадка,
#[derive(Deserialize, Debug)]
pub struct ShipMeasurements {
    length_design_waterline: f64,
    width_design_waterline: f64,
    number_spatiums: i64,


}

impl ShipMeasurements {
    pub fn new(length_design_waterline: f64, number_spatiums: i64, width_design_waterline: f64) -> Self {
        ShipMeasurements { length_design_waterline, number_spatiums, width_design_waterline }
    }

    pub fn length_spatium(&self) -> f64 {
        self.length_design_waterline / self.number_spatiums as f64
    }

    pub fn number_spatiums(&self) -> i64 {
        self.number_spatiums
    }
}