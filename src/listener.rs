use wasm_bindgen::{
    JsValue,
    JsCast,
    closure::{Closure, WasmClosure, IntoWasmClosure}
};
use workflow_core::id::Id;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("ClosureMismatch")]
    ClosureMismatch,

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

#[derive(Debug)]
pub struct Listener<T: ?Sized>{
    id: Id,
    closure: Arc<Mutex<Option<Arc<Closure<T>>>>>,
    closure_js_value: JsValue
}

impl<T:?Sized> Clone for Listener<T>{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            closure: self.closure.clone(),
            closure_js_value: self.closure_js_value.clone()
        }
    }
}

impl<T> Listener<T>
where T: ?Sized + WasmClosure + 'static
{

    pub fn new()->Self{
        Self{
            id: Id::new(),
            closure: Arc::new(Mutex::new(None)),
            closure_js_value: JsValue::null()
        }
    }

    pub fn callback<F>(&mut self, t:F)
    where F: IntoWasmClosure<T> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();
        //let c = ClosureType::WithResult(Arc::new(closure));

        *self.closure.lock().unwrap() = Some(Arc::new(closure));
        self.closure_js_value = closure_js_value;
    }

    pub fn with_callback<F>(t:F)->Self
    where F: IntoWasmClosure<T> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();
        Self{
            id: Id::new(),
            closure: Arc::new(Mutex::new(Some(Arc::new(closure)))),
            closure_js_value
        }
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
