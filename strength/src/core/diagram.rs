use crate::ship::{spatium::Spatium, loads::type_load::TypeLoad};


pub struct Diagram {
    spatiums: Vec<Spatium>,
    type_load: TypeLoad,
}


impl Diagram {
    pub fn new(spatiums: Vec<Spatium>, type_load: TypeLoad) -> Self {
        Diagram { spatiums, type_load }

    }
    pub fn draw(&self) {
        todo!()
    }
}

