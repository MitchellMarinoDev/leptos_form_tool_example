use leptos::prelude::*;
use leptos_form_tool::{styles::GridFormStyle, FormBuilder, FormToolData, ValidationBuilder};
use serde::{Deserialize, Serialize};

#[component]
pub fn HelloWorldFormPage() -> impl IntoView {
    let server_fn_action = ServerAction::<SubmitForm>::new();

    let form = HelloWorldFormData::default().get_action_form(
        server_fn_action,
        |_, _| {},
        GridFormStyle::default(),
        (),
    );
    let response = server_fn_action.value();

    view! {
      <div>
        <h1> "This is My Form!" </h1>
        // display the form
        {form.into_any()}
        // display the result from the server
        {move || response.get().map(|result| result.ok())}
      </div>
    }
}

/// All FormToolData implementors must also implement `Clone` and be `'static`.
/// `Serialize` and `Deserialize` are needed to send it to the server.
/// `Default` and `Debug` are not required, but helpful.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
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
        use leptos_form_tool::styles::GFStyleAttr::*;

        fb.heading(|h| h.title("Welcome"))
            .text_input(|t| {
                t.named("data[first]")
                    .labeled("First Name")
                    .getter(|fd| fd.first.clone())
                    .setter(|fd, value| fd.first = value)
                    // trim the string before writing to the field
                    .parse_trimmed()
                    .validation_fn(
                        // Using the ValidationBuilder to set a required field
                        ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.first.as_str())
                            .named("First Name")
                            .required()
                            .min_len(4)
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
                    .getter(|fd| fd.last.clone())
                    .setter(|fd, value| fd.last = value)
                    // dont trim the string, just write it to the field
                    .parse_from()
                    .validation_fn(|fd| {
                        if fd.last.is_empty() {
                            Err("Last Name must not be empty!".into())
                        } else if fd.last.len() < 4 {
                            Err("Last Name must not be less than 4 characters!".into())
                        } else {
                            Ok(())
                        }
                    })
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
                    .getter(|fd| fd.sport.clone())
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
                    .style(Width(5))
            })
            .button(|b| {
                b.text("Reset Form")
                    .action(|_event, form_data| {
                        form_data.set(Self::default());
                    })
                    .style(Width(1))
            })
            .submit(|s| s.text("Submit"))
    }
}

#[server(SubmitForm)]
async fn submit_form(data: HelloWorldFormData) -> Result<String, ServerFnError> {
    data.validate(()).map_err(ServerFnError::new)?;

    Ok(format!(
        "Hello {} {} ({}), You must like {}!",
        data.first, data.last, data.age, data.sport
    ))
}
