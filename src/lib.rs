use leptos::prelude::*;
use leptos::{ev::SubmitEvent, html};

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = signal("".to_string());

    view! {
        <FormSubmit
            set_input=set_input
            text="default"
        />
        <p>{input}</p>
    }
}

/// Form Submit that gives input to some outer scope
/// Leptos book chapters 3.3 and 3.6
#[component]
fn FormSubmit<'a>(
    /// write signal
    #[prop(into)]
    set_input: WriteSignal<String>,
    /// default text
    #[prop(optional)]
    text: &'a str,
) -> impl IntoView {
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_input.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                placeholder=text
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
    }
}
