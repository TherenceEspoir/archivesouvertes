use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Haldoc {
    docid: i64,
    label_s:Option<String>
}

impl Haldoc {
    pub fn new(docid:i64 , label_s :Option<String>) -> Self {
        Self {
            docid,
            label_s
        }
    }

    pub fn docid(&self) -> i64 {
        self.docid
    }

    pub fn label_s(&self) -> Option<String> {
        self.label_s.clone()
    }
}

impl fmt::Display for Haldoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Haldoc {{ docid: {}, label_s: {:?} }}", self.docid, self.label_s)
    }
}
