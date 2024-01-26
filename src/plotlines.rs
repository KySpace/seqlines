use crate::sequence::{AnalogSeq, DeviceDependentData, Sequence};

use plotly::common::{
    Fill, Font, Mode, Title,
};
use plotly::layout::{
    self, Axis, GridPattern, Layout, LayoutGrid, Margin, RangeSlider, Shape, ShapeLayer, ShapeLine, ShapeType
};
use plotly::{Bar, Plot, Scatter};
use plotly::color::{NamedColor, Color};

impl Sequence {
    pub fn to_html(&self) -> String {
        if let Some(anlg_sel) = &self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::Analog(anlg) = &seq.device_dependent 
            {Some(anlg)} else {None})
        .collect::<Vec<_>>().get(0) {
            let trace = trace_anlg(anlg_sel);
            let mut plot = Plot::new();
            plot.add_trace(trace);
            let range_slider = RangeSlider::new().visible(true);
            let layout = Layout::new().title(Title::new("Innocent Trial"))
            .plot_background_color(NamedColor::AliceBlue);
            plot.set_layout(layout);
            plot.to_html()
        }
        else {
            "Nothing here".to_string()
        }
    }
}

fn trace_anlg(anlg : &AnalogSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(anlg.times.clone(), anlg.times.clone())
}
