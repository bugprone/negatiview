use yew::prelude::*;

use crate::middlewares::error::Error;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component(ShowError)]
pub fn show_error(props: &Props) -> Html {
    if let Some(error) = &props.error {
        html! {
            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
                <span class="block sm:inline">
                    {
                        match error {
                            Error::UnprocessableEntity(error_info) => {
                                html! {
                                    <>
                                        {
                                            for error_info.errors.iter().map(|(key, value)| {
                                                html! {
                                                    <>
                                                        { key }
                                                        {
                                                            for value.iter().map(|err| {
                                                                html! {
                                                                    <> { ": " } { err } </>
                                                                }
                                                            })
                                                        }
                                                    </>
                                                }
                                            })
                                        }
                                    </>
                                }
                            }
                            _ => {
                                html! {
                                    { error.to_string() }
                                }
                            }
                        }
                    }
                </span>
            </div>
        }
    } else {
        html! {}
    }
}
