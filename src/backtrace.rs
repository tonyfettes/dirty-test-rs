extern crate alloc;

use alloc::vec::Vec;

#[derive(Clone)]
pub struct CallStack {
    pub calls: Vec<FuncCall>
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            calls: Vec::<FuncCall>::new(),
        }
    }
}

#[derive(Clone)]
pub struct FuncCall {
    pub name: &'static str
}

pub struct Backtrace {
    frames: Vec<BacktraceFrame>
}

pub struct BacktraceFrame {
    frame: Frame,
}

pub struct Frame {
}
