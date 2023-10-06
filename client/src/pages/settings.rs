use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::show_error::ShowError;
use crate::middlewares::context::use_user_context;
use crate::services::user::{current, save};
use crate::types::user::UserUpdateDto;
use crate::types::Wrapper;

#[function_component(Settings)]
pub fn setting() -> Html {
    let user_ctx = use_user_context();
    let update_dto = use_state(UserUpdateDto::default);
    let password = use_state(String::default);
    let user = use_async_with_options(
        async move { current().await },
        UseAsyncOptions::enable_auto(),
    );
    let update = {
        let update_dto = update_dto.clone();
        let password = password.clone();
        use_async(async move {
            let mut req = Wrapper::<UserUpdateDto> {
                data: (*update_dto).clone(),
            };
            if !(*password).is_empty() {
                req.data.password = Some((*password).clone());
            }
            save(req).await
        })
    };

    {
        let user = user.clone();
        let update_dto = update_dto.clone();
        use_effect_with(
            user,
            move |user| {
                if let Some(user) = &user.data {
                    update_dto.set(UserUpdateDto {
                        email: user.data.email.clone(),
                        display_name: user.data.display_name.clone(),
                        password: None,
                        biography: user.data.biography.clone(),
                        profile_image_url: user.data.profile_image_url.clone(),
                    });
                }
                || ()
            },
        );
    }

    {
        let user_ctx = user_ctx.clone();
        let update = update.clone();
        use_effect_with(
            update,
            move |update| {
                if let Some(user) = &update.data {
                    user_ctx.login_without_redirection(user.data.clone());
                }
                || ()
            },
        );
    }

    let onsubmit = {
        let update = update.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            update.run();
        })
    };

    let oninput_display_name = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.display_name = input.value();
            update_dto.set(dto);
        })
    };

    let oninput_email = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.email = input.value();
            update_dto.set(dto);
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let oninput_biography = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.biography = input.value();
            update_dto.set(dto);
        })
    };

    let oninput_profile_image_url = {
        let update_dto = update_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*update_dto).clone();
            dto.profile_image_url = input.value();
            update_dto.set(dto);
        })
    };

    let on_logout = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |_| {
            user_ctx.logout();
        })
    };

    html! {
        <div class="max-w-md mx-auto mt-12 mb-12">
            <h1 class="text-center text-xl font-semibold">{ "My Settings" }</h1>
            <ShowError error={user.error.clone()} />
            <ShowError error={update.error.clone()} />
            <form onsubmit={onsubmit} class="mt-4">
                <div class="mb-4">
                    <label for="email" class="block text-sm font-medium text-gray-700">
                        { "Email" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="email"
                        placeholder="Email"
                        value={update_dto.email.clone()}
                        oninput={oninput_email}
                    />
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-sm font-medium text-gray-700">
                        { "New Password"}
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="password"
                        placeholder="New Password"
                        value={(*password).clone()}
                        oninput={oninput_password}
                    />
                </div>
                <div class="mb-4">
                    <label for="display_name" class="block text-sm font-medium text-gray-700">
                        { "Display Name" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        placeholder="Display Name"
                        value={update_dto.display_name.clone()}
                        oninput={oninput_display_name}
                    />
                </div>
                <div class="mb-4">
                    <label for="profile_image_url" class="block text-sm font-medium text-gray-700">
                        { "Profile Image" }
                    </label>
                    <input
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        placeholder="URL to Profile Image"
                        value={update_dto.profile_image_url.clone()}
                        oninput={oninput_profile_image_url}
                    />
                </div>
                <div class="mb-4">
                    <label for="biography" class="block text-sm font-medium text-gray-700">
                        { "Bio" }
                    </label>
                    <textarea
                        class="mt-1 p-2 block w-full border rounded-md shadow-sm focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="text"
                        placeholder="About Yourself"
                        value={update_dto.biography.clone()}
                        oninput={oninput_biography}
                    />
                </div>
                <div class="flex justify-center">
                    <button class="w-full px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none"
                        type="submit"
                        disabled={user.loading || update.loading}>
                        {"Update"}
                    </button>
                </div>
                <p class="text-center text-sm">
                    { "Otherwise, you have the option to " }
                    <span class="text-blue-500 hover:underline" onclick={on_logout}>
                        { "logout" }
                    </span>
                    { "." }
                </p>
            </form>
        </div>
    }
}
