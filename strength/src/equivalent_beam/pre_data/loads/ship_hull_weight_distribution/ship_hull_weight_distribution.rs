use crate::equivalent_beam::{spatium::Spatium, pre_data::ship::ship::Ship};

pub struct ShipHullWeightDistribution {
    spatiums: Option<Vec<Spatium>>,
    ship: Ship,

}

impl ShipHullWeightDistribution {
    fn new(ship: Ship) -> Self {
        ShipHullWeightDistribution { spatiums: None, ship }

    }
}