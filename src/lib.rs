use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use web_sys::HtmlElement;
use starfish::*;
use js_sys;

static mut RUNNING: bool = false;
static mut RUN_COUNT: usize = 0;
static mut INPUT_READY: bool = false;

#[wasm_bindgen]
extern {
    fn getScriptVar() -> String;

    #[wasm_bindgen(js_namespace = LZString)]
    fn compressToEncodedURIComponent(s: &str) -> String;

    #[wasm_bindgen(js_namespace = LZString)]
    fn decompressFromEncodedURIComponent(s: &str) -> String;
}

#[wasm_bindgen]
pub fn run() {
    let script = getScriptVar();
    if &script != "" {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let script_box = get_textarea_element_by_id(&document, "script");
        script_box.set_value(&decompressFromEncodedURIComponent(&script));
    }
}

fn create_box(code_box: &CodeBox) -> String {
    let mut output = String::from("<pre><br>");
    let code_box = code_box.code_box();
    for y in 0..code_box.len() {
        for x in 0..code_box[y].len() {
            if x != 0 || y != 0 {
                output.push_str(&format!("<c id=\"{}x{}\">", x, y));
            } else {
                output.push_str(&format!("<c id=\"{}x{}\"  class=\"s\">", x, y));
            }
            output.push(code_box[y][x] as char);
            output.push_str("</c>");
        }
        output.push_str("<br>");
    }
    output.push_str("<br></pre>");
    return output;
}

fn get_input_element_by_id(document: &web_sys::Document, id: &str) -> HtmlInputElement {
    document.get_element_by_id(id).unwrap().
        dyn_into::<HtmlInputElement>().unwrap()
}

fn get_textarea_element_by_id(document: &web_sys::Document, id: &str) -> HtmlTextAreaElement {
    document.get_element_by_id(id).unwrap().
        dyn_into::<HtmlTextAreaElement>().unwrap()
}

fn get_html_element_by_id(document: &web_sys::Document, id: &str) -> HtmlElement {
    document.get_element_by_id(id).unwrap().
        dyn_into::<HtmlElement>().unwrap()
}

#[wasm_bindgen]
pub fn share_script() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let mut url = window.location().href().unwrap();
    if url.contains("?") {
        url = url.split("?").next().unwrap().to_string();
    }

    let share_field = get_input_element_by_id(&document, "sharefield");
    url.push_str("?script=");
    url.push_str(&compressToEncodedURIComponent(&get_textarea_element_by_id(&document, "script").value()));
    share_field.set_value(&url);

    let share_box = document.get_element_by_id("sharebox").unwrap();
    _ = share_box.set_attribute("style", "");
}

async fn sleep(ms: i32) -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

#[wasm_bindgen]
pub fn stop_script() {
    unsafe {
        RUN_COUNT += 1;
    }
}

#[wasm_bindgen]
pub fn collect_input() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let input_input = get_textarea_element_by_id(&document, "inputfield");
    let input_box = get_html_element_by_id(&document, "input");

    let mut out_text = input_box.inner_text();
    out_text.push_str(&input_input.value());

    input_box.set_inner_text(&out_text);
    input_input.set_value("");

    unsafe {
        INPUT_READY = true;
    }
}

#[wasm_bindgen]
pub async fn run_script() {
    let my_count: usize;
    unsafe {
        if RUNNING {
            return
        }
        RUNNING = true;
        my_count = RUN_COUNT;
    }

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let out_box = get_html_element_by_id(&document, "output");
    out_box.set_inner_text("");
    let mut out_string = String::new();

    let stack_box = get_html_element_by_id(&document, "stack");

    let stack = Stack::from_string(&get_input_element_by_id(&document, "initialstack").value()).unwrap_or(Stack::new(None));
    let mut code_box = starfish::CodeBox::new(&get_textarea_element_by_id(&document, "script").value(), stack, false);
    let delay: i32 = get_input_element_by_id(&document, "delay").value().parse().unwrap_or(0);

    let code_box_elem = get_html_element_by_id(&document, "codebox");
    code_box_elem.set_inner_html(&create_box(&code_box));

    let mut end = false;
    let mut output: Option<String>;
    let mut sleep_ms: f64;

    let mut last_x: usize = 0;
    let mut last_y: usize = 0;

    let input_box = get_html_element_by_id(&document, "input");

    while !end {
        unsafe {
            if INPUT_READY {
                INPUT_READY = false;
                let input_text = input_box.inner_text();
                code_box.inject_input(input_text.as_bytes().to_vec());
                input_box.set_inner_text("");
            }
        }

        (output, end, sleep_ms) = code_box.swim();
        match output {
            Some(val) => {
                if val.as_bytes()[0] == 13 {
                    out_string = String::new();
                } else {
                    out_string.push_str(&val);
                }
                out_box.set_inner_text(&out_string);
            },
            None => {}
        }

        if sleep_ms > 0.0 {
            _ = sleep(sleep_ms as i32).await;
        }
        if delay > 0 {
            _ = sleep(delay).await;
        }

        unsafe {
            if my_count != RUN_COUNT {
                break;
            }
        }

        if sleep_ms > 0.0 || delay > 0 {
            if !end {
                let last_pos = document.get_element_by_id(&format!("{}x{}", last_x, last_y)).unwrap();
                _ = last_pos.set_attribute("class", "");
                (last_x, last_y) = code_box.position();
                let cur_pos = document.get_element_by_id(&format!("{}x{}", last_x, last_y)).unwrap();
                if code_box.deep_sea() {
                    _ = cur_pos.set_attribute("class", "u");
                } else {
                    _ = cur_pos.set_attribute("class", "s");
                }
            }
            
            stack_box.set_inner_text(&code_box.string_stack());
        }
    }

    unsafe {
        RUNNING = false;
    }
}