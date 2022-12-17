mod file_dropper;
pub mod gen_url;
pub mod tranform_sgml_agent;
use crate::tranform_sgml_agent::SGMLTranformWorker;
use crate::{file_dropper::FileDropper, tranform_sgml_agent::SGMLTransformWorkerInput};
use gloo_console::log;
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

use yew_agent::Bridged;
/// Root html component, mount the file dropper and bridge the worker
#[function_component(App)]
pub fn app() -> Html {
    let worker = use_mut_ref(|| {
        SGMLTranformWorker::bridge(Rc::new(move |output| {
            // window.location.assign(fileObjUrl)
            // https://developer.mozilla.org/en-US/docs/Web/API/Location/assign
            // As the web browser cannot display ofx files, it is automatically downloaded
            window()
                .unwrap()
                .location()
                .assign(&output.url)
                .map_err(|err| log!("{:?}", err))
                .ok();
        }))
    });
    html! {
        <>
        <main>
    <section class="h-screen dark:bg-gray-900 grid-cols-3 grid-rows-3 grid">
         <div class="pb-8 px-4 my-auto mx-auto max-w-screen-md text-center lg:pb-16 lg:px-12 row-start-2 col-start-2">
            <FileDropper on_file_input={Callback::from(move |content: String| {
                // send a message to the worker with the SGML file content encoded as UTF8
               worker.borrow_mut().send(SGMLTransformWorkerInput{ file: content });
                })}
             />
        </div>
     </section>
     </main>
         <footer
           class="p-4 bg-white shadow md:flex md:items-center md:justify-between md:p-6 dark:bg-gray-800 absolute bottom-0 w-screen"
         >
           <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400"
             >{"© 2022 "}
             <a href="https://github.com/maxerbox/yew-trunk-web-worker-flowbite-tailwind-rust-wasm-example-app" class="hover:underline">{"GitHub"}</a> {" - Simon Sassi - Transforms OFX files for LBP"}
           </span>
           <script src="https://unpkg.com/flowbite@1.5.4/dist/flowbite.js"></script>
         </footer>
         </>
    }
}
