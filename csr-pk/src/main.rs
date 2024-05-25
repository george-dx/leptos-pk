use leptos::*;

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    style_top: u16,
    #[prop(into)]
    /// How much progress should be displayed.
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max="50"
            value=progress
            style="position: absolute"
            style:top=format!("{}px", style_top)
        ></progress>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>

        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
            }

            style="position: absolute"
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
        >
            "Click to Move"
        </button>

        <progress
            max="50"
            value=double_count
            style="position: absolute"
            style:top="75px"
        ></progress>
        <p>"Double Count: " {double_count}</p>

        <ProgressBar progress=count style_top=100/>
        <ProgressBar progress=Signal::derive(double_count) style_top=125/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
