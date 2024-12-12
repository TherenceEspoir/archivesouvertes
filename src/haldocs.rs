use crate::haldoc::Haldoc;

use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    start: i64,
    numFound: i64,
    docs: Vec<Haldoc>
}

impl Response {
    pub fn haldocs(&self) -> &Vec<Haldoc> {
        &self.docs
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn numFound(&self) -> i64 {
        self.numFound
    }
}