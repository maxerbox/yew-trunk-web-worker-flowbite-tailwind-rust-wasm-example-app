use chrono::prelude::*;
use default_env::default_env;
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use sgmlish::{transforms::Transform, SgmlEvent, SgmlFragment};
use web_sys::{File, FilePropertyBag, Url};
use yew_agent::{HandlerId, Public, WorkerLink};

use crate::gen_url::UrlExt;

static WORKER_PATH: &'static str = concat!(default_env!("TRUNK_PUBLIC_URL", "/"), "worker.js");

const INDEX_DATE_TOKEN_DELTA_FROM_TRNNAME_OPENTAG: usize = 7;
const INDEX_TEXTNODE_NAME_TOKEN_DELTA_FROM_TRNNAME_OPENTAG: usize = 2;
const WAIT_TIME_BEFORE_REVOKING: u32 = 1_000;

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
        let url = create_obj_url_from_fragment(fragment);
        let output = Self::Output { url };
        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        &WORKER_PATH
    }
}

fn create_obj_url_from_fragment(fragment: SgmlFragment) -> String {
    let parts = js_sys::Array::of1(&unsafe {
        js_sys::Uint8Array::view(fragment.to_string().as_bytes()).into()
    });
    let file = File::new_with_buffer_source_sequence_and_options(
        &parts,
        "export.ofx",
        FilePropertyBag::new().type_("application/x-ofx"),
    )
    .unwrap();
    let obj_url = UrlExt::create_object_url_with_file(&file).unwrap();
    let r = obj_url.clone();
    Timeout::new(WAIT_TIME_BEFORE_REVOKING, move || {
        Url::revoke_object_url(obj_url.as_str())
            .map_err(|err| println!("{:?}", err))
            .ok();
    })
    .forget();

    r
}

fn convert_date(fragment: SgmlFragment) -> SgmlFragment {
    let mut transform = Transform::new();

    for (i, event) in fragment.iter().enumerate().skip(1) {
        match event {
            SgmlEvent::OpenStartTag { name } if name == "NAME" => {
                match &fragment.as_slice()[i + INDEX_TEXTNODE_NAME_TOKEN_DELTA_FROM_TRNNAME_OPENTAG]
                {
                    SgmlEvent::Character(c) => {
                        apply_tranforms_for_transactions(c, &mut transform, i);
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

fn apply_tranforms_for_transactions(
    c: &std::borrow::Cow<str>,
    transform: &mut Transform,
    i: usize,
) {
    let (before, after) = c.rsplit_once(' ').unwrap();
    if let Ok(date) = NaiveDate::parse_from_str(after, "%d.%m.%y") {
        transform.remove_at(i - INDEX_DATE_TOKEN_DELTA_FROM_TRNNAME_OPENTAG);
        transform.insert_at(
            i - INDEX_DATE_TOKEN_DELTA_FROM_TRNNAME_OPENTAG,
            SgmlEvent::Character(date.format("%Y%m%d").to_string().into()),
        );
        transform.remove_at(i + INDEX_TEXTNODE_NAME_TOKEN_DELTA_FROM_TRNNAME_OPENTAG);
        transform.insert_at(
            i + INDEX_TEXTNODE_NAME_TOKEN_DELTA_FROM_TRNNAME_OPENTAG,
            SgmlEvent::Character(before.trim().to_owned().into()),
        );
    }
}
