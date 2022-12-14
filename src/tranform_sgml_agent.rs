use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sgmlish::{transforms::Transform, SgmlEvent, SgmlFragment};

use gloo_file::Blob;
use web_sys::Url;
use yew_agent::{HandlerId, Public, WorkerLink};

pub struct SGMLTranformWorker {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct SGMLTransformWorkerInput {
    pub file: String,
}

#[derive(Serialize, Deserialize)]
pub struct SGMLTransformWorkerOutput {
    pub url: String,
}

impl yew_agent::Worker for SGMLTranformWorker {
    type Input = SGMLTransformWorkerInput;
    type Message = ();
    type Output = SGMLTransformWorkerOutput;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!
        let parser = sgmlish::Parser::builder()
            .expand_marked_sections()
            .expand_entities(|entity| match entity {
                "lt" => Some("<"),
                "gt" => Some(">"),
                "amp" => Some("&"),
                "quot" => Some("\""),
                "apos" => Some("'"),
                _ => None,
            })
            .build();
        let fragment = parser.parse(&msg.file).ok().unwrap();

        let fragment = convert_date(fragment);
        let blob =
            Blob::new_with_options(fragment.to_string().as_bytes(), Some("application/x-ofx"));
        let obj_url = Url::create_object_url_with_blob(&web_sys::Blob::from(blob)).unwrap();

        let output = Self::Output {
            url: obj_url.to_string(),
        };
        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "/yew-trunk-web-worker-flowbite-tailwind-rust-wasm-example-app/worker.js"
    }
}

fn convert_date(fragment: SgmlFragment) -> SgmlFragment {
    let mut transform = Transform::new();

    for (i, event) in fragment.iter().enumerate().skip(1) {
        match event {
            SgmlEvent::OpenStartTag { name } if name == "NAME" => {
                match &fragment.as_slice()[i + 2] {
                    SgmlEvent::Character(c) => {
                        let (before, after) = c.rsplit_once(' ').unwrap();

                        if let Ok(date) = NaiveDate::parse_from_str(after, "%d.%m.%y") {
                            transform.remove_at(i - 7);
                            transform.insert_at(
                                i - 7,
                                SgmlEvent::Character(date.format("%Y%m%d").to_string().into()),
                            );
                            transform.remove_at(i + 2);
                            transform.insert_at(
                                i + 2,
                                SgmlEvent::Character(before.trim().to_owned().into()),
                            );
                        }
                    }
                    _ => {}
                }
            }
            SgmlEvent::Character(_)
            | SgmlEvent::ProcessingInstruction(_)
            | SgmlEvent::MarkupDeclaration { .. }
            | SgmlEvent::MarkedSection { .. }
            | SgmlEvent::CloseStartTag
            | SgmlEvent::EndTag { .. }
            | SgmlEvent::Attribute { .. }
            | SgmlEvent::XmlCloseEmptyElement
            | SgmlEvent::OpenStartTag { .. } => {}
        }
    }
    transform.apply(fragment)
}
