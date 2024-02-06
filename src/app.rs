use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_image::Image;
use leptos_meta::*;
use leptos_query::{QueryOptions, QueryResult, RefetchFn};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    leptos_query::provide_query_client();
    leptos_image::provide_image_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/reproducer-node-mismatch.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| {
                            view! {
                                <div>
                                    <HomePage/>
                                    <div>
                                        <div>
                                            // If you comment out the footer, there's no error??
                                            <Footer/>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let query = use_test_query();

    view! {
        <Transition fallback=|| ()>
            {move || {
                query
                    .data
                    .get()
                    .map(|image| {
                        view! { <Image src=image alt="Test Image" width=300 height=300 blur=true/> }
                    })
            }}

        </Transition>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div>
                <div>
                    <p>
                        failed: SSR and CSR elements have the same hydration key but different node kinds
                    </p>
                </div>
            </div>
        </footer>
    }
}

fn use_test_query() -> QueryResult<String, impl RefetchFn> {
    leptos_query::use_query(
        || (),
        |_| async { get_test_image().await.expect("Failed to get test image") },
        QueryOptions {
            default_value: None,
            stale_time: None,
            cache_time: None,
            refetch_interval: None,
            resource_option: leptos_query::ResourceOption::NonBlocking,
        },
    )
}
#[server(GetTestImage, "/api")]
pub async fn get_test_image() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok("cute_ferris.png".into())
}
