#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let input_str = use_state(cx, || String::from(""));
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
UserInput {text: input_str, on_submit: move |evt| println!("submit {}", evt)}
        })
}


// https://github.com/DioxusLabs/dioxus/issues/611
#[inline_props]
fn UserInput<'a>(
    cx: Scope<'a>,
    text: &'a UseState<String>,
    on_submit: EventHandler<'a, String>,
) -> Element {
    let clearing_state = &*cx.use_hook(|| std::cell::Cell::new(false));

    let inner_html = cx.use_hook(|| " ");
    if clearing_state.get() {
        *inner_html = "";
        cx.needs_update();
    }

    let res = render! {
        div {
            contenteditable: true,
            width: "200px",
            height: "200px",
            border: "1px solid black",
            "placeholder": "Type here",
            oninput: move |e| {
                if !clearing_state.get() {
                    println!("Input: {:?}", e);
                    text.set(e.value.clone());
                } else {
                    clearing_state.set(false);
                }
            },
            onkeyup: |e| {
                if e.data.key_code.eq(&KeyCode::Enter) && !e.data.shift_key {
                    on_submit.call(text.to_string());
                    text.set(String::from(""));
                    clearing_state.set(true);
                }
            },

            "dangerous_inner_html": "{inner_html}",
        }
    };

    clearing_state.set(false);
    *inner_html = " ";

    res
}