use crate::basic::{Component, Graphics};
use crate::Mutex;
use alloc::string::String;
use alloc::sync::Arc;

pub struct Label {
    inner: Mutex<LabelInner>,
}

struct LabelInner {
    text: String,
    graphic: Graphics,
    parent: Option<Arc<dyn Component>>,
}
