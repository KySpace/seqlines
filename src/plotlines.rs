use std::fmt::Debug;
use std::collections::HashMap;
use crate::sequence::{AnalogSeq, DDSSeq, DeviceDependentData, Sequence};

use plotly::common::{
    Fill, Font, Label, Mode, PlotType, Title
};
use plotly::layout::{
    self, Axis, GridPattern, Layout, LayoutGrid, Margin, RangeSlider, Shape, ShapeLayer, ShapeLine, ShapeType
};
use plotly::{Bar, Plot, Scatter, Trace};
use plotly::color::{NamedColor, Color};

#[derive(Clone, Copy, PartialEq, core::cmp::Eq, Hash)]
pub enum SubplotType {
    AnalogAmpl,
    DDSRFAmpl,
    DDSRFFreq,
    PLLVCOFreq,
    DDSRFShade,
    DigitalBlocks,
    DigitalBars,
}

pub type PlotMap<'a> = HashMap<SubplotType, Option<& 'a str>>;
pub type ScatLine = Box<Scatter<f64, f64>>;
pub type ScatLines = Vec<Box<Scatter<f64, f64>>>;

impl Sequence {
    pub fn to_html(&self) -> String {
        let mut plot: Plot = Plot::new();
        let mut plotmap :PlotMap  = HashMap::from([
            (SubplotType::AnalogAmpl        , Some("y1")),
            (SubplotType::DDSRFAmpl         , Some("y2")),
            (SubplotType::DDSRFFreq         , Some("y3")),
            (SubplotType::PLLVCOFreq        , Some("y4")),
            (SubplotType::DDSRFShade        , Some("y5")),
            (SubplotType::DigitalBlocks     , Some("y6")),
            (SubplotType::DigitalBars       , Some("y7")),            
        ]);
        for trace in self.traces_anlg(&plotmap).into_iter() {
            plot.add_trace(trace);
        }
        for trace in self.traces_dds(&plotmap).into_iter() {
            plot.add_trace(trace);
        }
        let range_slider = RangeSlider::new().visible(true);
        let layout = Layout::new().title(Title::new("Innocent Trial"))
        .x_axis(Axis::new().range_slider(range_slider))
        .plot_background_color(NamedColor::AliceBlue);
        plot.set_layout(layout);
        plot.to_html()
    }

    pub fn traces_anlg(&self, pm : &PlotMap) -> Vec<Box<Scatter<f64, f64>>> {
        let info_anlg : Vec<(&AnalogSeq, &String)> = self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::Analog(anlg) = &seq.device_dependent 
            {Some((anlg, &seq.name))} else {None})
        .collect::<Vec<_>>();
        let add_y_ampl_axis = add_axis(&SubplotType::AnalogAmpl, pm);
        info_anlg.iter().map(|&(d, s)| { add_y_ampl_axis(trace_anlg(d).name(s))} ).collect()
    }

    pub fn traces_dds(&self, pm : &PlotMap) -> Vec<Box<Scatter<f64, f64>>> {
        let info_ddsrf : Vec<(&DDSSeq, &String)> = self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::DDSRF(ddsrf) = &seq.device_dependent 
            {Some((ddsrf, &seq.name))} else {None})
        .collect::<Vec<_>>();
        let add_y_ampl_axis = add_axis(&SubplotType::DDSRFAmpl, pm);
        let add_y_freq_axis = add_axis(&SubplotType::DDSRFFreq, pm);
        let trace_ampl : ScatLines = info_ddsrf.iter().map(|&(d, s)| {
            add_y_ampl_axis(trace_ddsrf_ampl(d).name(s))})
            .collect();
        let trace_freq : ScatLines = info_ddsrf.iter().map(|&(d, s)| {
            add_y_freq_axis(trace_ddsrf_freq(d).name(s))})
            .collect();
        [trace_ampl, trace_freq].concat()
    }
}

// Should be able to avoid these lifetime annotation nonsense in the next edition of rust 
pub trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}

pub fn add_axis<'a>(plot_type : &'a SubplotType, map : &'a PlotMap) ->
    impl Fn(Box<Scatter<f64, f64>>) -> Box<Scatter<f64, f64>> + Captures<&'a ()> {
    move |trace| {
        if let Some(y_axis) = map[plot_type] {
            trace.y_axis(y_axis)
        } else { trace } }
}

fn trace_anlg(anlg : &AnalogSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(anlg.times.clone(), anlg.amplitude.clone())
}

fn trace_ddsrf_ampl(wave : &DDSSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(wave.times.clone(), wave.amplitude.clone())
}

fn trace_ddsrf_freq(wave : &DDSSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(wave.times.clone(), wave.frequency.clone())
}
