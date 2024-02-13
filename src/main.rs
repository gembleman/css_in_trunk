use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/>});
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, World!"</h1>
        </div>
    }
}
