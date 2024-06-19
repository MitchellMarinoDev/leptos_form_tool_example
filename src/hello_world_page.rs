use leptos::*;
use leptos_form_tool::{
    styles::GFStyleAttr::*, styles::GridFormStyle, FormBuilder, FormToolData, ValidationBuilder,
};
use serde::{Deserialize, Serialize};

#[component]
pub fn HelloWorldFormPage() -> impl IntoView {
    // create server action for submitting the form
    let server_fn_action = create_server_action::<SubmitHelloWorldForm>();

    // set the initial data, or start from `default()`
    let initial_form_data = HelloWorldFormData {
        first: "John".to_string(),
        last: "Doe".to_string(),
        age: 25,
        sport: "".to_string(),
    };

    // build the form
    let form = initial_form_data.get_form(server_fn_action, GridFormStyle::default(), ());

    let response = server_fn_action.value();

    view! {
        <div >
            <h1> "Hello World Form" </h1>
            <a href="/"> "return home" </a>
            <p> "This form mimics the form in getting_started.md" </p>
            // display the form
            {form}
            // display the result from the server
            {move || response.get().map(|result| result.ok())}
        </div>
    }
}

#[server(SubmitHelloWorldForm)]
async fn submit_hello_world_form(data: HelloWorldFormData) -> Result<String, ServerFnError> {
    data.validate(()).map_err(ServerFnError::new)?;

    Ok(format!(
        "Hello {} {} ({}), You must like {}!",
        data.first, data.last, data.age, data.sport
    ))
}
// all FormToolData implementors must also implement Clone and be 'static
// Default and Debug are not required, but helpful
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct HelloWorldFormData {
    first: String,
    last: String,
    age: u32,
    sport: String,
}

impl FormToolData for HelloWorldFormData {
    // The form style to use.
    type Style = GridFormStyle;
    // The external context needed for rendering the form.
    // In this case, nothing.
    type Context = ();

    fn build_form(fb: FormBuilder<Self>) -> FormBuilder<Self> {
        fb.heading(|h| h.title("Welcome"))
            .text_input(|t| {
                t.named("data[first]")
                    .labeled("First Name")
                    .getter(|fd| fd.first)
                    .setter(|fd, value| fd.first = value)
                    // trim the string before writing to the field
                    .parse_trimmed()
                    .validation_fn(
                        // Using the ValidationBuilder to set a required field
                        ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.first.as_str())
                            .named("First Name")
                            .required()
                            .build(),
                    )
                    // width out of 12
                    .style(Width(4))
                    // defines text that shows up when hovering over it
                    .style(Tooltip("Your given first name".to_string()))
            })
            .text_input(|t| {
                t.named("data[last]")
                    .labeled("Last Name")
                    .getter(|fd| fd.last)
                    .setter(|fd, value| fd.last = value)
                    // dont trim the string, just write it to the field
                    .parse_from()
                    .validation_fn(
                        // using the ValidationBuilder to set a required field
                        ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.last.as_str())
                            .named("Last Name")
                            .required()
                            .build(),
                    )
                    .style(Width(8))
                    .style(Tooltip("Your last name".to_string()))
            })
            // using the _cx varient allows access to the context
            // in this case, its `()` which doesnt help us that much.
            .stepper_cx(|s, _cx| {
                s.named("data[age]")
                    .labeled("Age")
                    .getter(|fd| fd.age)
                    .setter(|fd, value| fd.age = value)
                    // trim the string then try to parse to a `u32`
                    .parse_trimmed()
                    .validation_fn(move |fd| {
                        // defining a validation function with a closure
                        (fd.age > 13)
                            .then_some(())
                            .ok_or_else(|| String::from("Too Young"))
                    })
                    .style(Width(6))
                    .style(Tooltip("Your age in years".to_string()))
            })
            .select(|s| {
                s.named("data[sport]")
                    .labeled("Favorite Sport")
                    .getter(|fd| fd.sport)
                    .setter(|fd, value| fd.sport = value)
                    .parse_from()
                    // set the options for the select along with their values
                    .with_options_valued(
                        vec![
                            ("Football", "football"),
                            ("Soccer", "soccer"),
                            ("Ice Hockey", "ice_hockey"),
                            ("Golf", "golf"),
                        ]
                        .into_iter(),
                    )
                    .with_blank_option()
                    .style(Width(6))
            })
            .submit(|s| s.text("Submit"))
    }
}
