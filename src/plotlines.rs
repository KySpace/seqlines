use std::fmt::Debug;

use crate::sequence::{AnalogSeq, DeviceDependentData, Sequence};

use plotly::common::{
    Fill, Font, Label, Mode, Title
};
use plotly::layout::{
    self, Axis, GridPattern, Layout, LayoutGrid, Margin, RangeSlider, Shape, ShapeLayer, ShapeLine, ShapeType
};
use plotly::{Bar, Plot, Scatter};
use plotly::color::{NamedColor, Color};

impl Sequence {
    pub fn to_html(&self) -> String {
        let mut plot: Plot = Plot::new();
        for data in self.traces_anlg().into_iter() {
            let (trace, name) = data;
            let trace = trace.name(name);
            plot.add_trace(trace);
        }
        let range_slider = RangeSlider::new().visible(true);
        let layout = Layout::new().title(Title::new("Innocent Trial"))
        .plot_background_color(NamedColor::AliceBlue);
        plot.set_layout(layout);
        plot.to_html()
    }

    pub fn traces_anlg(&self) -> Vec<(Box<Scatter<f64, f64>>, &String)> {
        let info_anlg : Vec<(&AnalogSeq, &String)> = self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::Analog(anlg) = &seq.device_dependent 
            {Some((anlg, &seq.name))} else {None})
        .collect::<Vec<_>>();
        info_anlg.iter().map(|&(d, s)| (trace_anlg(d), s)).collect()
    }
}

fn trace_anlg(anlg : &AnalogSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(anlg.times.clone(), anlg.amplitude.clone())
}
