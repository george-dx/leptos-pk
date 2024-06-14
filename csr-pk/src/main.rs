use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

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
fn Basics() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count() * 2;
    let values = vec![0, 1, 2];
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters.map(|(count, set_count)| {
        view! {
            <li>
                <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
            </li>
        }
    })
    .collect_view();

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

        <ProgressBar progress=count style_top=300/>
        <ProgressBar progress=Signal::derive(double_count) style_top=325/>
        <p>{values.clone()}</p>
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
        <ul>{counter_buttons}</ul>
    }
}

#[component]
pub fn Iteration() -> impl IntoView {
    let (data, _set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);

    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        <For each=data key=|state| state.key.clone() let:child>
            <p>{child.value}</p>
        </For>
    }
}

#[component]
fn FormsAndInputControlled() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn FormsAndInputUncontrolled() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        let value = input_element().expect("<input> should be mounted").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn ControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() % 1 == 1;
    let message = move || {
        if is_odd() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };
    let message2 = move || {
        match value() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
        }
    };

    view! {
        <Show when=move || { value() > 5 } fallback=|| view! { "Small" }>
            "Big"
        </Show>
        <main>
            {move || match is_odd() {
                true if value() == 1 => view! { <pre>"One"</pre> }.into_any(),
                false if value() == 2 => view! { <p>"Two"</p> }.into_any(),
                _ => view! { <textarea>{value()}</textarea> }.into_any(),
            }}

        </main>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {
                                move || errors.get().into_iter().map(|(_, e)| view! {<li>{e.to_string()}</li>}).collect_view()
                            }
                        </ul>
                    </div>
                }
            >
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}

// #[component]
// pub fn ButtonB<F>(on_click: F) -> impl IntoView where F: Fn(MouseEvent) + 'static
// {
//     view! {
//         <button on:click=on_click>
//             "Toggle"
//         </button>
//     }
// }

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle"</button>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    render_prop: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
    {
        view! {
            <h2>"Render Prop"</h2>
            {render_prop()}

            <h2>"Children"</h2>
            {children()}
        }
    }

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect_view();

    view! {
        <ul>{children}</ul>
    }
}


#[component]
pub fn App() -> impl IntoView {
    // let (toggled, set_toggled) = create_signal(false);
    // view! {
    //     <p>"Toggled? " {toggled}</p>
    //     <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
    // }
    view! {
        <TakesChildren render_prop=|| view! {<p>"Hi, there!"</p>}>
        "Some text"
        <span>"A span"</span>
        </TakesChildren>

        <WrapsChildren>
        "A"
        "B"
        "C"
        </WrapsChildren>
    }
}



fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
