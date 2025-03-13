mod bili_api;
mod errors;

#[wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen]
pub async fn run(bvid: &str) {
    let result = bili_api::video_info(bvid).await;

    log(&format!("{result:?}"));
}

fn log(loginfo: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_with_str_1(loginfo)
        .unwrap();
}
