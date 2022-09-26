/**
 * Entry point file = lib.rs
 */

mod utils;
mod webgl2;
mod game_of_life;

use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;
use crate::webgl2::starto;
use crate::utils::log;

use gloo::{events::EventListener};
use wasm_bindgen::JsCast;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    log("inside main");
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_id("p");
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    log("before starto");
    starto()?;
    log("before event");
    event();

    Ok(())
}

pub fn event() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let p = document.get_element_by_id("p").unwrap().dyn_into::<web_sys::HtmlParagraphElement>().unwrap();
    let on_click = EventListener::new(&p, "click", move |event| { 
        let click_event = event.clone().dyn_into::<web_sys::MouseEvent>().unwrap();
        let mut click_event_str = String::from("");
        click_event_str.push_str(&click_event.type_());
        log(&format!("got event, {click_event_str}"));
    });
    on_click.forget();
}
