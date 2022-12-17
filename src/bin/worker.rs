use lbp_convert::tranform_sgml_agent::SGMLTranformWorker;
use yew_agent::PublicWorker;

// wasm worker main
fn main() {
    SGMLTranformWorker::register();
}
