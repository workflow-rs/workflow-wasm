use wasm_bindgen::{JsValue, JsCast, closure::Closure, convert::FromWasmAbi};
use workflow_core::id::Id;
use std::sync::{Arc, Mutex};
//use crate::Result;
type Result<T> = std::result::Result<T,JsValue>;
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
pub enum ClosureType<T>{
    WithResult(Arc<Closure<dyn FnMut(T)->Result<()>>>),
    WithoutResult(Arc<Closure<dyn FnMut(T)>>)
}

impl<T> Clone for ClosureType<T>{
    fn clone(&self) -> Self {
        match self {
            ClosureType::WithResult(c)=>{
                ClosureType::WithResult(c.clone())
            }
            ClosureType::WithoutResult(c)=>{
                ClosureType::WithoutResult(c.clone())
            }
        }
    }
}

#[derive(Debug)]
pub struct Listener<T>{
    id: Id,
    closure: Arc<Mutex<Option<ClosureType<T>>>>,
    closure_js_value: JsValue
}

impl<T> Clone for Listener<T>{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            closure: self.closure.clone(),
            closure_js_value: self.closure_js_value.clone()
        }
    }
}

impl<T> Listener<T>
where T: Sized + FromWasmAbi + 'static
{

    pub fn new()->Self{
        Self{
            id: Id::new(),
            closure: Arc::new(Mutex::new(None)),
            closure_js_value: JsValue::null()
        }
    }

    pub fn callback<F>(&mut self, t:F)
    where F: FnMut(T)->Result<()> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();
        let c = ClosureType::WithResult(Arc::new(closure));

        *self.closure.lock().unwrap() = Some(c);
        self.closure_js_value = closure_js_value;
    }

    pub fn callback_without_result<F>(&mut self, t:F)
    where F: FnMut(T) + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();
        let c = ClosureType::WithoutResult(Arc::new(closure));

        *self.closure.lock().unwrap() = Some(c);
        self.closure_js_value = closure_js_value;
    }

    pub fn with_callback<F>(t:F)->Self
    where F: FnMut(T)->Result<()> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();
        let c = ClosureType::WithResult(Arc::new(closure));
        Self{
            id: Id::new(),
            closure: Arc::new(Mutex::new(Some(c))),
            closure_js_value
        }
    }

    pub fn into_js<J>(&self) -> &J
    where J: JsCast
    {
        self.closure_js_value.as_ref().unchecked_ref()
    }

    pub fn closure(&self) -> std::result::Result<ClosureType<T>, Error>
    {
        match self.closure.lock(){
            Ok(locked)=>{
                match locked.as_ref(){
                    Some(c)=>{
                        Ok((*c).clone())
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

    pub fn closure_with_result(&self) -> std::result::Result<Arc<Closure<dyn FnMut(T)->Result<()> + 'static>>, Error>
    {
        match self.closure()?{
            ClosureType::WithResult(c)=>{
                Ok(c)
            }
            ClosureType::WithoutResult(_)=>{
                return Err(Error::ClosureMismatch)
            }
        }
    }

    pub fn closure_without_result(&self) -> std::result::Result<Arc<Closure<dyn FnMut(T) + 'static>>, Error>
    {
        match self.closure()?{
            ClosureType::WithResult(_)=>{
                return Err(Error::ClosureMismatch)
            }
            ClosureType::WithoutResult(c)=>{
                Ok(c)
            }
        }
    }
}
