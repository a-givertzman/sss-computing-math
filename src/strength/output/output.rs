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
            x.push(spatiun.start_coord);
            y.push(spatiun.start_value);
            x.push(spatiun.end_coord);
            y.push(spatiun.end_value);
        }
        let step = (self.spatiums[0].end_coord - self.spatiums[0].end_coord).abs();
        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name("name")
        .line(Line::new().shape(LineShape::Linear));  
        let mut plot = Plot::new();
        let layout = Layout::new()
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new("Lightweight intensity"));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}

