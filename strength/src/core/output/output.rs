use crate::ship::spatium::Spatium;

use super::type_output::TypeOutput;


pub struct Output {
    spatiums: Vec<Spatium>,
    type_output: TypeOutput,
}


impl Output {
    pub fn new(spatiums: Vec<Spatium>, type_output: TypeOutput) -> Self {
        Output { spatiums, type_output }

    }
    
    pub fn draw(&self) {
        todo!()
    }
}

