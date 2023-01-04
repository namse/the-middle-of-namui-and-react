use namsex_core::*;
use std::collections::VecDeque;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlElement};

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

    spawn_local(async move {
        main().await;
    });

    Ok(())
}

async fn main() {
    run::<MyRoot, _>(
        MyRootProps {},
        |str| console::log_1(&JsValue::from_str(str)),
        sync_tree_to_platform,
    )
    .await;
}

fn sync_tree_to_platform(tree: &mut TreeNode) {
    let parent = web_sys::window()
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
                props: _,
                children,
            } => {
                for child in children {
                    queue.push_back(child);
                }
            }
            TreeNode::PlatformNode {
                platform_node,
                rendered_real_dom,
                prev_platform_node,
            } => match rendered_real_dom {
                Some(rendered_real_dom_mut) => match prev_platform_node {
                    Some(prev_platform_node) => {
                        let rendered_real_dom =
                            rendered_real_dom_mut.downcast_ref::<HtmlElement>().unwrap();
                        match (&platform_node, &prev_platform_node) {
                            (PlatformNode::Button(button), PlatformNode::Button(prev_button)) => {
                                update_button(button, prev_button, rendered_real_dom);
                            }
                            (PlatformNode::Text(text), PlatformNode::Text(prev_text)) => {
                                todo!()
                            }
                            _ => {
                                rendered_real_dom.remove();

                                let real_dom = create_real_dom(platform_node);
                                parent.append_child(&real_dom).unwrap();
                                *rendered_real_dom_mut = Box::new(real_dom);
                            }
                        }
                    }
                    None => {}
                },
                None => {
                    let real_dom = create_real_dom(platform_node);
                    parent.append_child(&real_dom).unwrap();
                    *rendered_real_dom = Some(Box::new(real_dom));
                }
            },
        }
    }
}

fn update_button(button: &Button, prev_button: &Button, rendered_real_dom: &HtmlElement) {
    let mut casted_real_dom = None;

    let mut get_casted_real_dom = || {
        if casted_real_dom.is_none() {
            casted_real_dom = Some(
                rendered_real_dom
                    .dyn_ref::<web_sys::HtmlButtonElement>()
                    .unwrap(),
            );
        }
        casted_real_dom.unwrap()
    };

    if button.text != prev_button.text {
        let casted_real_dom = (get_casted_real_dom)();
        casted_real_dom.set_text_content(Some(&button.text));
    }
    if button.id != prev_button.id {
        let node_id = button.id;
        let casted_real_dom = (get_casted_real_dom)();
        casted_real_dom.set_onclick(Some(
            Closure::wrap(Box::new(move || {
                console::log_1(&JsValue::from_str("Button clicked!"));

                namsex_core::event::send(node_id, ButtonEvent::Click)
            }) as Box<dyn FnMut()>)
            .into_js_value()
            .unchecked_ref(),
        ));
    }
}

fn create_real_dom(platform_node: &PlatformNode) -> HtmlElement {
    console::log_1(&JsValue::from_str("create_real_dom"));
    match platform_node {
        PlatformNode::Button(button) => {
            // 어떻게 과거꺼를 가져와서 작업해?
            // 과거꺼를 저장해야해. Option<Box<dyn Any>>의 형식으로.
            // 과거꺼가 있으면 뭘 어떻게 바꿔야해?
            // 바뀐 값만 설정해야해. 바뀌었는지 알려면 과거의 virtual dom이 무엇인지 알아야해.

            let button_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("button")
                .unwrap()
                .dyn_into::<web_sys::HtmlButtonElement>()
                .unwrap();

            let node_id = button.id;

            button_element.set_text_content(Some(&button.text));
            button_element.set_onclick(Some(
                Closure::wrap(Box::new(move || {
                    console::log_1(&JsValue::from_str("Button clicked!"));

                    namsex_core::event::send(node_id, ButtonEvent::Click)
                }) as Box<dyn FnMut()>)
                .into_js_value()
                .unchecked_ref(),
            ));
            button_element.into()
        }
        PlatformNode::Text(text) => {
            console::log_1(&JsValue::from_str(&format!("Attach text node: {:?}", text)));
            todo!()
        }
    }
}
