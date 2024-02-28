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
            Ok(seq) => view! { <iframe class="w-screen h-screen text-amber-700" srcdoc={seq.to_html()} /> }.into_view(),
            Err(err) => view! {  <div class="text-red-700">{err}</div>  }.into_view(),
        }
    };
    let on_click = move |_| file_content.set("".to_string());
    view! {
        <div class="text-center">
        <button on:click=on_click class="bg-rose-200 px-4 h-9 inline-flex items-center rounded border border-gray-300 shadow-sm text-sm font-medium text-neutral-500"> Clear </button>
        <p class="text-teal-700"> {file_name} </p>
        <Suspense fallback=move || view! { <p class="text-amber-700">"Loading (Suspense Fallback)..."</p> }>                
            {view_from_str(&file_content.get().unwrap_or_else(|| {"Error".to_string()}))}
        </Suspense>
        </div>
    }
}

#[component]
pub fn DropZone(set_file_info : WriteSignal<Vec<web_sys::File>>) -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);    
    let drop_zone_el = create_node_ref::<Div>();   

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |e| set_file_info.set(e.files))
            .on_enter(move |_| set_dropped(false)),
    );
    
    // let on_click = move |_| set_dropped(false);

    view! {
        <div 
            node_ref=drop_zone_el
            class="max-w-96 m-8 border-2 border-dashed border-gray-400 rounded-md
                    px-4 py-4 justify-center text-center text-lg text-cyan-600"
            class:drop-zone-over=move || is_over_drop_zone()
            >
            <p>Drop files Here!</p>
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
    let (file_info, set_file_info) = 
        create_signal::<Vec<web_sys::File>>(vec![]);
    provide_context(file_info);
    provide_context(set_file_info);
    view! {
        <div class="flex flex-col flex-nowrap items-center">
            <div class="flex flex-row flex-nowrap items-center">
                <img width="64" src="img/leptos-use-logo.svg" alt="Drop me"/>
                <DropZone set_file_info/>
            </div>
            <For each=file_info key=|f| f.name() let:file>
                <PlotLines file/>
            </For>
        </div>
    }
}
