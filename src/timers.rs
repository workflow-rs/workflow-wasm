use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JsValue {0:?}")]
    JsValue(JsValue),
    
    #[error("Invalid interval handle")]
    InvalidIntervalHandle,

    #[error("Invalid timeout handle")]
    InvalidTimeoutHandle,
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

pub mod native {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {

        #[wasm_bindgen (catch, js_name = setInterval)]
        pub fn set_interval(closure: &Closure<dyn FnMut()>, timeout: u32 ) -> std::result::Result<u32, JsValue>;

        #[wasm_bindgen (catch, js_name = clearInterval)]
        pub fn clear_interval(interval: u32) -> std::result::Result<(), JsValue>;

        #[wasm_bindgen (catch, js_name = setTimeout)]
        pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> std::result::Result<u32, JsValue>;

        #[wasm_bindgen (catch, js_name = clearTimeout)]
        pub fn clear_timeout(interval: u32) -> std::result::Result<(), JsValue>;

    }
}

#[derive(Clone)]
pub struct IntervalHandle(Arc<Mutex<u32>>);

impl Drop for IntervalHandle
{
    fn drop(&mut self) {
        let handle = self.0.lock().unwrap();
        if *handle != 0 {
            native::clear_interval(*handle).expect("Unable to clear interval");
        }
    }
}

#[derive(Clone)]
pub struct TimeoutHandle(Arc<Mutex<u32>>);

impl Drop for TimeoutHandle
{
    fn drop(&mut self) {
        let handle = self.0.lock().unwrap();
        if *handle != 0 {
            native::clear_timeout(*handle).expect("Unable to clear timeout");
        }
    }
}


pub fn set_interval(closure: &Closure<dyn FnMut()>, timeout: u32 ) -> Result<IntervalHandle,Error> {
    let handle = native::set_interval(closure,timeout)?;
    Ok(IntervalHandle(Arc::new(Mutex::new(handle))))
}

pub fn clear_interval(handle: &IntervalHandle) -> Result<(),Error> {
    let mut handle = handle.0.lock().unwrap();
    if *handle != 0 {
        native::clear_timeout(*handle)?;
        *handle = 0;
        Ok(())
    } else {
        Err(Error::InvalidIntervalHandle)
    }
}

pub fn set_timeout(closure: &Closure<dyn FnMut()>, timeout: u32) -> Result<TimeoutHandle,Error> {
    let handle = native::set_timeout(closure,timeout)?;
    Ok(TimeoutHandle(Arc::new(Mutex::new(handle))))    
}

pub fn clear_timeout(handle: &TimeoutHandle) -> Result<(),Error> {
    let mut handle = handle.0.lock().unwrap();
    if *handle != 0 {
        native::clear_timeout(*handle)?;
        *handle = 0;
        Ok(())
    } else {
        Err(Error::InvalidTimeoutHandle)
    }
}
