use gloo::console::log;
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use stylist::{style, yew::styled_component};

mod components;

use components::atoms::main_title::MainTitle;


#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

// #[function_component(App)]
#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = style!(
        r#"
            h1 {
                color: yellow;
            }
        "#
    ).unwrap();

    let name = "Brooks";
    let my_object = MyObject{
        username: name.to_string(),
        favorite_language: "rust".to_string()
    };

    log!("My name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let tasks = vec!["record", "shopping", "work"];

    html! {
        <div class={stylesheet}>
            <MainTitle title="Hello Yew" />
            <p>{"Hi There!"}</p>

            if true {
                <p>{"If statement"}</p>
            }

            <ul>
            {tasks.iter().map(|task| html!{<li>{task}</li>}).collect::<Html>()}
            </ul>

            <ul class={css!("color: lightgreen;")}>
                {list_to_html(tasks)}
            </ul>
        </div>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|task| html!{<li>{task}</li>}).collect()
}
