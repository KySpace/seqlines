use crate::error_template::{AppError, ErrorTemplate};
use crate::sequences;
use crate::plotlines;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Div;
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use leptos_use::docs::{demo_or_body, BooleanDisplay};

#[component]
pub fn PlotLines(
        #[prop()]
        file : web_sys::File
    ) -> impl IntoView {
    let file_name = file.name();
    let file_content = create_resource(
        | | (), 
        move |_| { 
            let text = file.text();
            async move { wasm_bindgen_futures::JsFuture::from(text).await.unwrap().as_string().unwrap() }
        });
    let view_from_str = |file : &str| {
        match sequences::Sequence::from_json(file) {
            Ok(seq) => view! { <iframe class="w-screen" srcdoc={seq.to_html()} /> }.into_view(),
            Err(err) => view! {  <div>{err}</div>  }.into_view(),
        }
    };
    let on_click = move |_| file_content.set("".to_string());
    view! {
        <div class="w-screen h-full">
        <button on:click=on_click> Clear </button>
        <p> {file_name} </p>
            <Suspense fallback=move || view! { <p>"Loading (Suspense Fallback)..."</p> }>                
                {view_from_str(&file_content.get().unwrap_or_else(|| {"Error".to_string()}))}
            </Suspense>
        </div>
    }
}

#[component]
pub fn DropZone() -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);    
    let drop_zone_el = create_node_ref::<Div>();   

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |_| set_dropped(true))
            .on_enter(move |_| set_dropped(false)),
    );
    
    // let on_click = move |_| set_dropped(false);

    view! {
        <div>
            <div class="w-screen h-screen relative">
                <p>Drop files into dropZone</p>
                <img width="64" src="img/leptos-use-logo.svg" alt="Drop me"/>
                <div
                    node_ref=drop_zone_el
                    class="flex flex-col w-screen h-screen bg-gray-400/10 justify-center items-center pt-6"
                >
                    <div>is_over_drop_zone: <BooleanDisplay value=is_over_drop_zone/></div>
                    <div>dropped: <BooleanDisplay value=dropped/></div>
                    <div class="flex flex-wrap justify-center items-center w-screen h-screen bg-gray-900/5">
                        <For each=files key=|f| f.name() let:file>
                            <div class="w-200px bg-black-200/10 ma-2 pa-6">
                                <p>Name: {file.name()}</p>
                                <p>Size: {file.size()}</p>
                                <p>Type: {file.type_()}</p>
                                <p>Last modified: {file.last_modified()}</p>
                            </div>
                            <PlotLines file=file/>
                        </For>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/try-leptos6.css"/>

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
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <DropZone/>
    }
}
