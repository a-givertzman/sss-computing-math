use plotly::{Scatter, common::{Mode, Line, LineShape, Font, Title}, Plot, Layout, layout::{Legend, TraceOrder}};

use crate::strength::ship::spatium::Spatium;

use super::type_output::TypeOutput;


pub struct Output {
    pub spatiums: Vec<Spatium>,
    pub type_output: TypeOutput,
}


impl Output {
    pub fn new(spatiums: Vec<Spatium>, type_output: TypeOutput) -> Self {
        Output { spatiums, type_output }

    }
    
    pub fn draw(&self) {
        let mut x = vec![];
        let mut y = vec![];
        for spatiun in &self.spatiums {
            x.push(spatiun.x1);
            y.push(spatiun.f_x1);
            x.push(spatiun.x2);
            y.push(spatiun.f_x2);
        }
        let step = (self.spatiums[0].x2 - self.spatiums[0].x1).abs();
        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name("name")
        .line(Line::new().shape(LineShape::Linear));  
        let mut plot = Plot::new();
        let layout = Layout::new()
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new("Lightweight tonnage intensity"));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}

