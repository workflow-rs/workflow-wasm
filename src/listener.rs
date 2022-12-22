//! 
//! Callback is a callback wrapper that owns a given rust closure
//! meant to be bound to JavaScript as callbacks. 
//! 
//! 

use wasm_bindgen::{
    JsValue,
    JsCast,
    closure::{Closure, WasmClosure, IntoWasmClosure}
};
pub use workflow_core::id::Id;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{

    #[error("String {0:?}")]
    String(String),

    #[error("JsValue {0:?}")]
    JsValue(JsValue),

    #[error("LockError: Unable to lock closure, {0:?}")]
    LockError(String),

    #[error("ClosureNotIntialized, Please use `listener.callback()`")]
    ClosureNotIntialized
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

impl From<String> for Error{
    fn from(str: String) -> Self {
        Self::String(str)
    }
}

pub type CallbackClosure<T> = dyn FnMut(T) -> std::result::Result<(), JsValue>;
pub type CallbackClosureWithoutResult<T> = dyn FnMut(T);

pub trait Listener{
    fn get_id(&self)->Id;
}

#[derive(Debug)]
pub struct Callback<T: ?Sized>{
    id: Id,
    closure: Arc<Mutex<Option<Arc<Closure<T>>>>>,
    closure_js_value: JsValue
}

impl<T> Listener for Callback<T>
where T: ?Sized + WasmClosure + 'static
{
    fn get_id(&self)->Id{
        self.id
    }
}


impl<T:?Sized> Clone for Callback<T>{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            closure: self.closure.clone(),
            closure_js_value: self.closure_js_value.clone()
        }
    }
}

impl<T> Callback<T>
where T: ?Sized + WasmClosure + 'static
{

    pub fn new()->Self{
        Self{
            id: Id::new(),
            closure: Arc::new(Mutex::new(None)),
            closure_js_value: JsValue::null()
        }
    }

    pub fn with_closure<F>(t:F)->Self
    where F: IntoWasmClosure<T> + 'static
    {
        let mut listener = Self::new();
        listener.set_closure(t);

        listener
    }

    pub fn set_closure<F>(&mut self, t:F)
    where F: IntoWasmClosure<T> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();

        *self.closure.lock().unwrap() = Some(Arc::new(closure));
        self.closure_js_value = closure_js_value;
    }

    pub fn into_js<J>(&self) -> &J
    where J: JsCast
    {
        self.closure_js_value.as_ref().unchecked_ref()
    }

    pub fn closure(&self) -> std::result::Result<Arc<Closure<T>>, Error>
    {
        match self.closure.lock(){
            Ok(locked)=>{
                match locked.as_ref(){
                    Some(c)=>{
                        Ok(c.clone())
                    }
                    None=>{
                        return Err(Error::ClosureNotIntialized)
                    }
                }
            }
            Err(err)=>{
                return Err(Error::LockError(err.to_string()))
            }
        }
    }
}
