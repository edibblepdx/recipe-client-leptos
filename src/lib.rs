mod recipe;

use crate::recipe::Recipe;

use leptos::prelude::*;
use leptos::{ev::SubmitEvent, html};

#[component]
pub fn App() -> impl IntoView {
    view! { <FetchRecipe/> }
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
    placeholder: &'a str,
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
                placeholder=placeholder
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
    }
}

/// based on Leptos crate fetch example
#[component]
fn FetchRecipe() -> impl IntoView {
    let (endpoint, set_endpoint) = signal::<String>("random".to_string());
    let recipe = LocalResource::new(move || recipe::fetch(endpoint.get()));

    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let (input, set_input) = signal("".to_string());

    view! {
        <div>
            <Transition fallback=|| view! { <div>"Loading..."</div> }>
                <ErrorBoundary fallback>
                {move || Suspend::new(async move {
                    recipe.map(|r| {
                        match r {
                            Ok(r) => view! { <CommonRecipe r=r/> }.into_any(),
                            Err(_) => view! { <DefaultRecipe/> }.into_any(),
                        }
                    })
                })}
                </ErrorBoundary>
            </Transition>
        </div>
        <div class="new-recipe">
            <h4>"New Recipe"</h4>
            <FormSubmit
                set_input=set_input
                placeholder="cuisine (optional)"
            />
            // change the endpoint whenever input changes
            {move || set_endpoint.set(format!("cuisine/{}", input.get()))}
        </div>
    }
}

#[component]
fn CommonRecipe<'a>(r: &'a Recipe) -> impl IntoView {
    view! {
        <div class="recipe">
            <span class="name">{r.name.clone()}</span><br/>
            <span class="field">"cuisine: "{r.cuisine.clone()}</span><br/>
            <span class="field">"cook time: "{r.cooking_time_minutes}</span><br/>
            <span class="field">"prep time: "{r.prep_time_minutes}</span><br/>
            <span class="field">"servings: "{r.servings}</span><br/>
            <span class="field">"calories: "{r.calories_per_serving}</span><br/>
            <h4>"Ingredients"</h4>
            <ul>
                { r.ingredients.iter().map(|i| {
                    view! { <li> { i.clone() } </li> }
                }).collect::<Vec<_>>()}
            </ul>
            <h4>"Dietary Restrictions"</h4>
            <ul>
                { r.dietary_restrictions.iter().map(|i| {
                    view! { <li> { i.clone() } </li> }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
fn DefaultRecipe() -> impl IntoView {
    view! {
        <div class="recipe">
            <span class="name">"Not Found"</span><br/>
            <span class="field">"cuisine: None"</span><br/>
            <span class="field">"cook time: None"</span><br/>
            <span class="field">"prep time: None"</span><br/>
            <span class="field">"servings: None"</span><br/>
            <span class="field">"calories: None"</span><br/>
            <h4>"Ingredients"</h4>
            <ul>
                <li>"None"</li>
            </ul>
            <h4>"Dietary Restrictions"</h4>
            <ul>
                <li>"None"</li>
            </ul>
        </div>
    }
}
