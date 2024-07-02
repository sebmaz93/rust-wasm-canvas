use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

const FPS: u32 = 300;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = Rc::new(window().unwrap());
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let window_clone = window.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let color = format!(
            "rgb({}, {}, {})",
            js_sys::Math::random() * 255.0,
            js_sys::Math::random() * 255.0,
            js_sys::Math::random() * 255.0
        );

        context.set_fill_style(&JsValue::from_str(&color));
        context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

        window_clone
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                1000 / FPS as i32,
            )
            .expect("should register `requestAnimationFrame` OK");
    }) as Box<dyn FnMut()>));

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            1000 / FPS as i32,
        )
        .expect("should register `requestAnimationFrame` OK");

    Ok(())
}
