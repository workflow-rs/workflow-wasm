pub use crate::callback::{
    Callback,
    CallbackClosure,
    CallbackClosureWithoutResult,
    CallbackId,
    CallbackMap,
    //CallbackTrait0,
    //CallbackTrait1,
    //CallbackTrait2,
    AsCallback
};

pub use crate::timers::{
    IntervalHandle,
    TimeoutHandle,
    set_interval,
    set_timeout,
};

pub use workflow_wasm_macros::callback;