use plotly::{Scatter, common::{Mode, Line, LineShape, Font, Title}, Plot, Layout, layout::{Legend, TraceOrder, Axis, RangeMode, LayoutGrid, GridPattern}};

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
        let mut x: Vec<f64> = vec![];
        let mut y = vec![];
        for spatiun in &self.spatiums {
            x.push(spatiun.x1);
            y.push(spatiun.f_x1);
            x.push(spatiun.x2);
            y.push(spatiun.f_x2);
        }
        let trace1 = Scatter::new(x, y)
        .mode(Mode::LinesMarkers)
        .name("name")
        .line(Line::new().shape(LineShape::Linear));
        let mut plot = Plot::new();
        let layout = Layout::new()
            .y_axis(Axis::new().range_mode(RangeMode::ToZero).dtick(2.0))
            .x_axis(Axis::new().dtick(5.0))
            .legend(Legend::new().font(Font::new().size(16)))
            .title(Title::new("Lightweight intensity"));
        plot.add_trace(trace1);
        plot.set_layout(layout);
        plot.show();
    }
}

