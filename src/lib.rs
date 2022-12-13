use std::rc::Rc;
pub mod file_dropper;
pub mod tranform_sgml_agent;
use crate::{file_dropper::FileDropper, tranform_sgml_agent::SGMLTransformWorkerInput};

use crate::tranform_sgml_agent::SGMLTranformWorker;
use gloo_console::log;
use web_sys::window;
use yew::prelude::*;

use yew_agent::Bridged;
#[function_component(App)]
pub fn app() -> Html {
    let worker = use_mut_ref(|| {
        SGMLTranformWorker::bridge(Rc::new(move |output| {
            log!(&output.url);
            window()
                .unwrap()
                .location()
                .assign(&output.url)
                .map_err(|err| log!("{:?}", err))
                .ok();
        }))
    });
    html! {
    <section class="h-screen dark:bg-gray-900 grid-cols-3 grid-rows-3 grid">
         <div class="py-8 px-4 my-auto mx-auto max-w-screen-md text-center lg:py-16 lg:px-12 row-start-2 col-start-2">
            <FileDropper on_file_input={Callback::from(move |content: String| {
               worker.borrow_mut().send(SGMLTransformWorkerInput{ file: content });
                })}
             />
        </div>
     </section>
    }
}
