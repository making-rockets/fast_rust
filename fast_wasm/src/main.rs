use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_console::log;

mod util;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn index_component() -> Html {
    html! {
    <>

    <div>{"后台管理入口"}</div>
    <Login/>
    </>
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserLogin {
    user_name: Option<String>,
    password: Option<String>,
}


 

impl UserLogin {
    fn new(user_name: Option<String>, password: Option<String>) -> Self {
        Self {
            user_name,
            password,
        }
    }
}

pub enum Msg {
    UpdateValue(String),
}

#[function_component(Login)]
pub fn login() -> Html {
    let login_info = use_state(UserLogin::default);

    let oninput_user_name = {

        let login_info_clone = login_info.clone();

        Callback::from(move |input_event: InputEvent| {
            let input: HtmlInputElement = input_event.target_unchecked_into();
            let value = input.value();
            login_info_clone.user_name = Some(value)
        })
    };

    let oninput_password = Callback::from(|input_event: InputEvent| {
        let input: HtmlInputElement = input_event.target_unchecked_into();
        let value = input.value();
        log!("text", value);
    });

    let button_submit = Callback::from(|mouse_event: MouseEvent| {
        let button = mouse_event.button();
    });

    html! {
            <>
                <div>
                <form id= "login_form">
                  <span>{"用户名："}</span>  <input id= "user_name" oninput={oninput_user_name}/>
                  <span>{"密  码："}</span>  <input id = "password" oninput = {oninput_password} />
                  <button  onclick = {button_submit} >{"点击登录"}</button>
                </form>
                </div>
            </>

    }
}
