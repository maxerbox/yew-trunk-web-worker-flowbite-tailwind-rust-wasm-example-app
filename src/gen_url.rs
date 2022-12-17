//! This module generate a new type UrlExt with wasm_bindgen
//! The reason is that web_sys does not supports generating bindings for createObjectURL() with a file argument.

#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
use web_sys::File;
#[wasm_bindgen]
extern "C" {

    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = URL , typescript_type = "URL")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Url` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)"]
    #[doc = ""]
    pub type UrlExt;

    # [wasm_bindgen (catch , static_method_of = UrlExt, js_class = "URL" , js_name = createObjectURL)]
    #[doc = "The `createObjectURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)"]
    #[doc = ""]
    pub fn create_object_url_with_file(blob: &File) -> Result<String, JsValue>;
}
