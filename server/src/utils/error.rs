use std::{fmt::Display, rc::Rc, sync::Arc};

#[derive(Clone)]
pub struct Error<E: std::error::Error> {
    pub msg: String,
    pub source: Arc<Option<E>>,
}

impl<E: std::error::Error> Error<E> {
    pub fn from(e: E) -> Self {
        Self {
            msg: e.to_string().clone(),
            source: Arc::new(Some(e)),
        }
    }

    pub fn new(msg: String) -> Self {
        Self {
            source: Arc::new(None),
            msg,
        }
    }
}

#[derive(Debug)]
pub struct PureError {
    pub msg: String,
}

impl PureError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for PureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.msg)
    }
}

impl std::error::Error for PureError {}
