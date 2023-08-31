use diddoc_legacy::aries::diddoc::AriesDidDoc;
use messages::msg_fields::protocols::connection::request::Request;

use crate::protocols::connection::trait_bounds::{BootstrapDidDoc, HandleProblem, TheirDidDoc, ThreadId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Requested {
    pub(crate) did_doc: AriesDidDoc,
    pub(crate) thread_id: String,
    pub(crate) request: Request,
}

impl Requested {
    pub fn new(did_doc: AriesDidDoc, thread_id: String, request: Request) -> Self {
        Self {
            did_doc,
            thread_id,
            request,
        }
    }
}

impl TheirDidDoc for Requested {
    fn their_did_doc(&self) -> &AriesDidDoc {
        &self.did_doc
    }
}

impl BootstrapDidDoc for Requested {}

impl ThreadId for Requested {
    fn thread_id(&self) -> &str {
        &self.thread_id
    }
}

impl HandleProblem for Requested {}
