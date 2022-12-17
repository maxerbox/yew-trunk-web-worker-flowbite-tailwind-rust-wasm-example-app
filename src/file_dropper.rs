use std::mem;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{EventTarget, File, HtmlInputElement};
use yew::prelude::*;

pub enum FileDropperMsg {
    /// Whenever a file is selected or not
    SelectFile(Option<File>),
}

#[derive(Properties, PartialEq)]
pub struct FileDropperProps {
    /// Event fired when a valid file is selected.
    /// Event contains file contents read with Blob.text(), an UTF8 string
    pub on_file_input: Callback<String>,
}

impl Component for FileDropper {
    type Message = FileDropperMsg;

    type Properties = FileDropperProps;

    fn create(_ctx: &Context<Self>) -> Self {
        // At the beginning, there was no file...
        Self { file: None }
    }
    #[allow(unused_must_use)]
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileDropperMsg::SelectFile(file) => {
                let on_file_input = ctx.props().on_file_input.clone();
                let f1 = move |text: JsValue| {
                    let text = text.as_string().unwrap();
                    on_file_input.emit(text);
                };
                let f1 = Closure::once(f1);
                let ret = (&f1).clone();
                if let Some(file) = &file {
                    // read the blob text (we could have used a filereader here, but it's more convenient to use a js promise),
                    // once the js promise is resolved, then emit the event
                    file.text().then(ret);
                }
                mem::forget(f1);
                // forget to call the destructor of the Closure, else the function will be destroyed when out of scope.
                // NOTE: That's a closure called only once, so it will cause memory leaks if file.text() promise fails
                self.file = file;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="flex items-center justify-center w-full">
            <label for="dropzone-file" class="flex py-11 px-2 flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                <div class="flex flex-col items-center justify-center pt-5 pb-6">
                    <svg aria-hidden="true" class="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold">{ "Cliquez ici " }</span>{ "ou glissez-déposez" }</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{ "Fichier OFX" }</p>
                </div>
                <input id="dropzone-file" type="file" class="hidden" accept="application/x-ofx, .ofx" onchange={ctx.link().callback(
                    |e: Event| {
                    // when dispatched does the target get added.
                    let target: Option<EventTarget> = e.target();
                    // cast into an HtmlInputElement, not really safe
                    let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                        // if there is a file, emit a new message with the file
                        if let Some(input) = input {
                            let files = input.files();
                            if let Some(files) = files {
                                if files.length() > 0 {
                                    let file = files.get(0);
                                    return FileDropperMsg::SelectFile(file);
                                }
                            }
                        }
                        FileDropperMsg::SelectFile(None)
                    }
                )}/>
            </label>
        </div>
        }
    }
}

/// A file dropper html component
pub struct FileDropper {
    file: Option<File>,
}
