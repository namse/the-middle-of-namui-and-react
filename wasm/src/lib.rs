use namsex_core::*;
use std::collections::VecDeque;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    main();
    Ok(())
}

fn main() {
    run::<MyRoot, _>(
        MyRootProps {},
        |str| console::log_1(&JsValue::from_str(str)),
        sync_tree_to_platform,
    );
}

fn sync_tree_to_platform(tree: &TreeNode) {
    let mut parent = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(tree);

    while let Some(node) = queue.pop_front() {
        match node {
            TreeNode::Component {
                component: _,
                children,
            } => {
                for child in children {
                    queue.push_back(child);
                }
            }
            TreeNode::PlatformNode { platform_node } => match platform_node {
                PlatformNode::Button(button) => {
                    console::log_1(&JsValue::from_str(&format!(
                        "Attach button node: {:?}",
                        button
                    )));
                    let button_element = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_element("button")
                        .unwrap()
                        .dyn_into::<web_sys::HtmlButtonElement>()
                        .unwrap();

                    button_element.set_text_content(Some(&button.text));
                    button_element.set_onclick(Some(
                        Closure::wrap(Box::new(move || {
                            console::log_1(&JsValue::from_str("Button clicked!"));
                        }) as Box<dyn FnMut()>)
                        .into_js_value()
                        .unchecked_ref(),
                    ));
                    parent.append_child(&button_element).unwrap();
                }
                PlatformNode::Text(text) => {
                    console::log_1(&JsValue::from_str(&format!("Attach text node: {:?}", text)));
                }
            },
        }
    }
}
