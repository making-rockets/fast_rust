use gloo_console::log;
use serde::{Deserialize, Serialize};

use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::util::request::request_post;
mod components;
mod util;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn index_component() -> Html {
    html! {
    <>

    <div class="text-3xl  center   mx-auto" >{"后台管理入口"}</div>
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
    LoginResult(String),
}

#[function_component(Login)]
pub fn login() -> Html {
    //禁止js 右键
    let function = js_sys::Function::default().bind(&JsValue::FALSE);
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_oncontextmenu(Some(&function));

    let user_name_state = use_state(String::new);
    let password_state = use_state(String::new);

    let oninput_user_name = {
        let user_name_state_clone = user_name_state.clone();
        Callback::from(move |input_event: InputEvent| {
            let input: HtmlInputElement = input_event.target_unchecked_into();
            let value = input.value();
            log!("user_name = {}", &value);
            user_name_state_clone.clone().set(value);
        })
    };

    let oninput_password = {
        let password_state_clone = password_state.clone();
        Callback::from(move |input_event: InputEvent| {
            let input: HtmlInputElement = input_event.target_unchecked_into();
            let value = input.value();
            log!("password = {}", &value);
            password_state_clone.set(value);
        })
    };
    let button_submit = Callback::from(move |mouse_event: MouseEvent| {
        let button = mouse_event.button();
        log!("button = ", button);
        match button {
            0 => {
                //  发送请求
                // 获取user_name and password

                let user_login = UserLogin::new(
                    Some(user_name_state.to_string()),
                    Some(password_state.to_string()),
                );

                // let requeset_body = JsValue::from_serde(&user_login).unwrap();
                let request_body = serde_json::json!(&user_login);

                mouse_event.prevent_default();
                let future = async move {
                    let login_result =
                        request_post("http://localhost:3000/admin/user/add_user", &request_body)
                            .await;
                    match login_result {
                        Ok(result) => {
                            log!(&result);
                            Msg::LoginResult(result);
                        }
                        Err(e) => {
                            //log!(e);
                        }
                    }
                };

                wasm_bindgen_futures::spawn_local(future);
            }
            1 => {
                log!("点击中键");
            }
            2 => {
                log!("点击→键");
            }
            _ => {
                mouse_event.prevent_default();
            }
        }
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
