use crate::error_template::{AppError, ErrorTemplate};
use crate::seqserv::SequenceRef;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use plotly::common::{
    Fill, Font, Mode, Title,
};
use plotly::layout::{
    Axis, GridPattern, Layout, LayoutGrid, Margin, Shape, ShapeLayer, ShapeLine,
    ShapeType, RangeSlider,
};
use plotly::{Bar, Plot, Scatter};
use plotly::color::{NamedColor, Color};

fn test(name : &str) -> String {
    let trace1 = Scatter::new(vec![1., 1.5, 2.], vec![1, 2, 1]).name("(1,1)");
    let trace2 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,1)")
        .x_axis("x1")
        .y_axis("y2");
    let trace3 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("(1,2,2)")
        .x_axis("x1")
        .y_axis("y4");
    let trace4 = Scatter::new(vec![1, 2], vec![1, 2])
        .name("{(2,1), (2,2)}")
        .x_axis("x1")
        .y_axis("y3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]).show_grid(false).visible(false))
        .y_axis(Axis::new().range(vec![0.0, 3.5]).range_mode(plotly::layout::RangeMode::NonNegative).show_line(false))
        .plot_background_color(NamedColor::LightGrey);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(1.)
            .y0(1.)
            .x1(2.)
            .y1(3.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(3.)
            .y0(1.)
            .x1(6.)
            .y1(2.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(2.))
            .fill_color(NamedColor::LightSkyBlue),
    );
    let range_slider = RangeSlider::new().visible(true);
    let layout = Layout::new().title(Title::new(name))
        .x_axis(Axis::new().domain(&[0., 1.]).anchor("x1").range_slider(range_slider).show_line(true).mirror(true))
        .y_axis(Axis::new().domain(&[0., 0.2]).anchor("x1").show_line(true).mirror(true))
        .x_axis2(Axis::new().domain(&[0., 1.]).anchor("y2"))
        .y_axis2(Axis::new().domain(&[0.5, 0.75]).anchor("x1"))
        .x_axis3(Axis::new().domain(&[0., 1.]).anchor("y3"))
        .y_axis3(Axis::new().domain(&[0.25, 0.45]).anchor("x1"))
        .x_axis4(Axis::new().domain(&[0., 1.]).anchor("y4"))
        .y_axis4(Axis::new().domain(&[0.8, 1.]).anchor("x1"))
        .plot_background_color(NamedColor::AliceBlue);
    plot.set_layout(layout);
    plot.to_html()
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/seqlines.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let sequence : SequenceRef = expect_context::<SequenceRef>();
    let plot = (*sequence.clone().lock().unwrap()).to_html();
    view! {
        <div inner_html=plot/>
    }
}
