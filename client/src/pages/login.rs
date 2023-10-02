use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::middlewares::context::use_user_context;
use crate::routes::AppRoute;
use crate::services::user::login;
use crate::types::user::LoginDto;
use crate::types::Wrapper;

#[function_component(Login)]
pub fn login_page() -> Html {
    let user_ctx = use_user_context();
    let login_dto = use_state(LoginDto::default);

    let login = {
        let login_dto = login_dto.clone();
        use_async(async move {
            login(Wrapper::<LoginDto> { data: (*login_dto).clone() }).await
        })
    };

    use_effect_with_deps(
        move |login| {
            if let Some(resp) = &login.data {
                user_ctx.login(resp.data.clone());
            }
            || ()
        },
        login.clone(),
    );

    let onsubmit = {
        let login = login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            login.run();
        })
    };

    let oninput_email = {
        let login_dto = login_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*login_dto).clone();
            dto.email = input.value();
            login_dto.set(dto);
        })
    };

    let oninput_password = {
        let login_dto = login_dto.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut dto = (*login_dto).clone();
            dto.password = input.value();
            login_dto.set(dto);
        })
    };

    html! {
        <div class="max-w-md mx-auto mt-12 mb-12">
            <h1 class="text-center text-xl font-semibold mb-4">{ "Login" }</h1>
            <form onsubmit={ onsubmit }>
                <div class="mb-4">
                    <label for="email" class="block text-sm font-medium text-gray-700">
                        { "Email" }
                    </label>
                    <input
                        class="mt-1 p-2 border rounded w-full"
                        type="email"
                        value={ login_dto.email.clone() }
                        oninput={ oninput_email }
                        />
                </div>
                <div class="mb-4">
                    <label for="password" class="block text-sm font-medium text-gray-700">
                        { "Password" }
                    </label>
                    <input
                        class="mt-1 p-2 border rounded w-full"
                        type="password"
                        value={ login_dto.password.clone() }
                        oninput={ oninput_password }
                        />
                </div>
                <div class="flex justify-center">
                    <button type="submit" class="w-full px-4 py-2 bg-indigo-600 border rounded-md text-white hover:bg-indigo-700 focus:ring focus:ring-indigo-300 focus:outline-none">
                        { "Login" }
                    </button>
                </div>
                <p class="text-center text-sm">
                    { "Don't have an account? " }
                    <span class="text-blue-500 hover:underline">
                        <Link<AppRoute> to={AppRoute::SignUp}>
                            { "Sign Up" }
                        </Link<AppRoute >>
                    </span>
                </p>
            </form>
        </div>
    }
}
