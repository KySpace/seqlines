use std::fmt::Debug;
use std::collections::HashMap;
use crate::sequences::{AnalogSeq, DDSSeq, DeviceDependentData, DigitalSeq, Sequence, VCOSeq};

use leptos::with;
use plotly::common::{
    Fill, Font, Label, Line, Mode, PlotType, Title
};
use plotly::layout::{
    self, Axis, AxisConstrain, GridPattern, Layout, LayoutGrid, Margin, RangeSlider, Shape, ShapeLayer, ShapeLine, ShapeType
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
    DigitalLines,
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
            (SubplotType::DigitalLines      , Some("y7")),            
        ]);
        let traces = [ 
            self.traces_anlg(&plotmap), 
            self.traces_dds(&plotmap),
            self.traces_dig(&plotmap),
            self.traces_vco(&plotmap),
            ].concat();
        for trace in traces {
            plot.add_trace(trace);
        }
        let range_slider = RangeSlider::new().visible(true);
        let layout = Layout::new().title(Title::new("Innocent Trial"))
        .x_axis(Axis::new().range_slider(range_slider))
        .plot_background_color(NamedColor::AliceBlue)
        .height(1000);
        let layout = adjust_y_height(layout);
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

    pub fn traces_vco(&self, pm : &PlotMap) -> Vec<Box<Scatter<f64, f64>>> {
        let info_vco : Vec<(&VCOSeq, &String)> = self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::PLLVCO(vco) = &seq.device_dependent 
            {Some((vco, &seq.name))} else {None})
        .collect::<Vec<_>>();
        let add_y_ampl_axis = add_axis(&SubplotType::PLLVCOFreq, pm);
        info_vco.iter().map(|&(d, s)| { add_y_ampl_axis(trace_vco_freq(d).name(s))} ).collect()
    }

    pub fn traces_dig(&self, pm : &PlotMap) -> Vec<Box<Scatter<f64, f64>>> {
        let info_dig : Vec<(&DigitalSeq, &String, u8)> = self.seq_channel.iter().filter_map(|seq| 
            if let DeviceDependentData::Digital(dig) = &seq.device_dependent 
            {Some((dig, &seq.name, seq.index_sigchan))} else {None})
        .collect::<Vec<_>>();
        let add_y_ampl_axis = add_axis(&SubplotType::DigitalLines, pm);
        info_dig.iter().map(|&(d, s, c)| { add_y_ampl_axis(trace_dig_lines(d,c).name(s))} ).collect()
    }
}

// Should be able to avoid these lifetime annotation nonsense in the next edition of rust 
pub trait Captures<U> {}
impl<T: ?Sized, U> Captures<U> for T {}

pub fn add_axis<'a>(plot_type : &'a SubplotType, map : &'a PlotMap) ->
    impl Fn(Box<Scatter<f64, f64>>) -> Box<Scatter<f64, f64>> + Captures<&'a ()> {
    move |trace| {
        match map[plot_type] { 
            Some(y_axis) => trace.y_axis(y_axis),
            None => trace 
        }
    }
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

fn trace_vco_freq(wave : &VCOSeq) -> Box<Scatter<f64, f64>> {
    Scatter::new(wave.times.clone(), wave.frequency.clone())
}

fn trace_dig_lines(wave : &DigitalSeq, i : u8) -> Box<Scatter<f64, f64>> {
    let y = wave.value.clone().iter().map(|v| (i + (if *v {1} else {0})) as f64).collect::<Vec<_>>();
    Scatter::new(wave.times.clone(), y)
        .mode(Mode::LinesMarkers)
        .line(Line::new().shape(plotly::common::LineShape::Hv))
}

pub fn adjust_y_height(layout : Layout) -> Layout {
    let height = &[300.,400.,300.,500.,600.,0.,100.,1600.];
    let height_tot : f64 = height.iter().sum();
    let h_gap = 40.;
    let mut height_cum = [(0., 0.);8];
    let height_tot = height_cum.iter_mut().enumerate()
        .fold(0., |cum, (i, h)| {
            let cum = cum + h_gap;
            let cum_new = cum + height[i];
            *h = (cum, cum_new); 
            cum_new });
    let domain : Vec<[f64;2]> = height_cum.iter().map(|(b, t)| [b / height_tot, t / height_tot]).collect();
    println!("Domain Size : {:?}", domain);
    core::array::from_fn::<_,8,_>(|i| i).iter()
        .fold(layout, |l, i| {
        let axis = Axis::new()
                .domain(&domain[*i])
                .anchor("x1")
                .title(Title::new(&i.to_string()));
        l.new_axis_idx(*i, axis)
    })    
}

pub trait AccessAxis {
    fn new_axis(self, name_axis : Option<&str>, axis : Axis) -> Self;
    fn new_axis_idx(self, idx_axis : usize, axis : Axis) -> Self;
}

impl AccessAxis for Layout {
    fn new_axis(self, name_axis : Option<&str>, axis : Axis) -> Self {
        match name_axis {
            Some("y1") => self.y_axis (axis),
            Some("y2") => self.y_axis2(axis),
            Some("y3") => self.y_axis3(axis),
            Some("y4") => self.y_axis4(axis),
            Some("y5") => self.y_axis5(axis),
            Some("y6") => self.y_axis6(axis),
            Some("y7") => self.y_axis7(axis),
            Some("y8") => self.y_axis8(axis),
            _ => self
        }
    }
    fn new_axis_idx(self, idx_axis : usize, axis : Axis) -> Self {
        match idx_axis {
            1 => self.y_axis (axis),
            2 => self.y_axis2(axis),
            3 => self.y_axis3(axis),
            4 => self.y_axis4(axis),
            5 => self.y_axis5(axis),
            6 => self.y_axis6(axis),
            7 => self.y_axis7(axis),
            8 => self.y_axis8(axis),
            _ => self
        }
    }
}