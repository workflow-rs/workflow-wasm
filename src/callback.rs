//! 
//! [`callback`](self) module provides [`Callback`] struct that helps registering
//! Rust closures as JavaScript callbacks.
//! 

use wasm_bindgen::{
    JsValue,
    JsCast,
    closure::{Closure, WasmClosure, IntoWasmClosure},
    convert::{FromWasmAbi, ReturnWasmAbi}
};
use workflow_core::id::Id;
use std::{sync::{Arc, Mutex, MutexGuard}, collections::HashMap};
use thiserror::Error;

/// `u64`-based Callback Id (alias of [`workflow_core::id::Id`]).
pub type CallbackId = Id;

/// Errors produced by the [`callback`](self) module
#[derive(Error, Debug)]
pub enum Error{

    /// Custom error message
    #[error("String {0:?}")]
    String(String),

    /// Error contains a JsValue
    #[error("JsValue {0:?}")]
    JsValue(JsValue),

    /// LockError message resulting from Mutex lock failure ([`std::sync::PoisonError`])
    #[error("LockError: Unable to lock closure, {0:?}")]
    LockError(String),

    #[error("ClosureNotIntialized, Please use `callback.set_closure()`")]
    /// Results from trying to access a closure value when the closure is not initialized.
    ClosureNotInitialized
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

impl From<String> for Error{
    fn from(str: String) -> Self {
        Self::String(str)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Callback Closure that produces a [`wasm_bindgen::JsValue`] error
pub type CallbackClosure<T> = dyn FnMut(T) -> std::result::Result<(), JsValue>;
/// Callback Closure that yields no [`std::result::Result`]
pub type CallbackClosureWithoutResult<T> = dyn FnMut(T);

/// Trait allowing to bind a generic [`Callback`] struct
/// with a [`CallbackId`] identifier.
pub trait AsCallback {
    fn get_id(&self)->CallbackId;
}

///
/// [`Callback`] is a struct that owns a given Rust closure 
/// meant to be bound to JavaScript as a callback.
///
pub struct Callback<T: ?Sized>{
    id: CallbackId,
    closure: Arc<Mutex<Option<Arc<Closure<T>>>>>,
    closure_js_value: JsValue
}

impl<T: ?Sized> std::fmt::Debug for Callback<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback{{ id:\"{}\" }}", self.id)
    }
}

impl<T> AsCallback for Callback<T>
where T: ?Sized + WasmClosure + 'static
{
    fn get_id(&self)->CallbackId{
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

impl<T> Default for Callback<T>
where T: ?Sized + WasmClosure + 'static
{
    fn default() -> Self {
        Self {
            id: CallbackId::new(),
            closure: Arc::new(Mutex::new(None)),
            closure_js_value: JsValue::null()
        }
    }
}
macro_rules! create_fns {
    ($(
        ($name: ident, $($var:ident)*)
    )*) => ($(        
        pub fn $name<$($var,)* R>(callback:T)->Callback<dyn FnMut($($var,)*)->R>
        where 
            T: 'static + FnMut($($var,)*)->R,
            $($var: FromWasmAbi + 'static,)*
            R: ReturnWasmAbi + 'static
        {
            Callback::create(callback)
        }
        
    )*)
}
impl<T> Callback<T>{
    create_fns! {
        (new_with_args_0, )
        (new_with_args_1, A)
        (new_with_args_2, A B)
        (new_with_args_3, A B C)
        (new_with_args_4, A B C D)
        (new_with_args_5, A B C D E)
        (new_with_args_6, A B C D E F)
        (new_with_args_7, A B C D E F G)
        (new_with_args_8, A B C D E F G H)
    }

    /// Create a new [`Callback`] instance with the given closure.
    pub fn new<A, R>(callback: T) -> Callback<dyn FnMut(A) -> R>
    where
        T: 'static + FnMut(A) -> R,
        A: FromWasmAbi + 'static,
        R: ReturnWasmAbi + 'static,
    {
        Callback::create(callback)
    }
}

impl<T> Callback<T>
where T: ?Sized + WasmClosure + 'static
{
    /// Create a new [`Callback`] instance with the given closure.
    pub fn create<F>(t:F)->Self
    where F: IntoWasmClosure<T> + 'static
    {
        let mut callback = Callback::<T>::default();
        callback.set_closure(t);

        callback
    }

    /// Set closure in the given [`Callback`] instance.
    pub fn set_closure<F>(&mut self, t:F)
    where F: IntoWasmClosure<T> + 'static
    {
        let closure = Closure::new(t);
        let closure_js_value = closure.as_ref().clone();

        *self.closure.lock().unwrap() = Some(Arc::new(closure));
        self.closure_js_value = closure_js_value;
    }

    /// Obtain a [`wasm_bindgen::JsCast`] value for this callback.
    pub fn into_js<J>(&self) -> &J
    where J: JsCast
    {
        self.closure_js_value.as_ref().unchecked_ref()
    }

    /// Obtain an [`std::sync::Arc`] of the given closure.
    /// Returns [`Error::ClosureNotInitialized`] if the closure is `None`.
    pub fn closure(&self) -> Result<Arc<Closure<T>>>
    {
        match self.closure.lock(){
            Ok(locked)=>{
                match locked.as_ref(){
                    Some(c)=>{
                        Ok(c.clone())
                    }
                    None=>{
                        return Err(Error::ClosureNotInitialized)
                    }
                }
            }
            Err(err)=>{
                return Err(Error::LockError(err.to_string()))
            }
        }
    }
}

impl<T> AsRef<JsValue> for Callback<T> 
where T: ?Sized + WasmClosure + 'static
{
    fn as_ref(&self)-> &JsValue{
        self.closure_js_value.as_ref().unchecked_ref()
    }
}

impl<T> Into<JsValue> for Callback<T> 
where T: ?Sized + WasmClosure + 'static
{
    fn into(self) -> JsValue{
        self.closure_js_value.unchecked_into()
    }
}


impl<T> AsRef<js_sys::Function> for Callback<T>
where T: ?Sized + WasmClosure + 'static
{
    fn as_ref(&self)-> &js_sys::Function{
        self.closure_js_value.as_ref().unchecked_ref()
    }
}

/// Collection of callbacks contained in a [`std::collections::HashMap`].
#[derive(Clone)]
pub struct CallbackMap {
    inner : Arc<Mutex<HashMap<CallbackId, Arc<dyn AsCallback>>>>
}

impl std::fmt::Debug for CallbackMap{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallbackMap{{...}}")
    }
}

impl CallbackMap {
    /// Create a new [`CallbackMap`] instance.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    /// Get access to the [`std::sync::MutexGuard`] owning the inner [`std::collections::HashMap`].
    pub fn inner(&self) -> MutexGuard<HashMap<CallbackId, Arc<dyn AsCallback>>> {
        self.inner.lock().unwrap()
    }

    /// Insert a new callback into the collection
    pub fn insert<L>(&self, callback:L)->Result<()>
    where
        L: Sized + AsCallback + 'static
    {
        let id = callback.get_id();

        self.inner
            .lock()
            .map_err(|err| Error::LockError(err.to_string()))?
            .insert(id, Arc::new(callback));

        Ok(())
    }

    /// Remove a callback from the collection
    pub fn remove(&self, id:&CallbackId)->Result<Option<Arc<dyn AsCallback>>> {
        let v = self.inner
            .lock()
            .map_err(|err| Error::LockError(err.to_string()))?
            .remove(id);
        Ok(v)
    }

}

/// 
/// creates a [Callback] instance 
/// by inspecting a given closure signature
/// 
/// ```
/// // include dependencies
/// use workflow_wasm::prelude::*;
/// ```
/// <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
///
/// **Warning**: the [`macro@callback`] macro expects to receive a closure as an argument
/// and will use this closure's signature to determine which [`Callback`] binding function
/// to use.  However, if you declare a closure as a variable and then try to pass
/// it to the [`macro@callback`] macro, the macro will fail with an error as follows: 
/// "closure is expected to take 1 argument"
/// 
/// </pre></div>
/// 
/// - #### If passing closure as variable, it will accept only 1 argument:
/// ```
/// let closure_as_variable = |value:bool|{
///     ...
/// };
/// let callback = callback!(closure_as_variable);
/// ```
/// The above code will create callback like this:
/// ```
/// let callback = Callback::new(closure_as_variable);
/// ```
/// 
/// - #### Examples of incorrect use: 
///
/// ```compile_fail
/// // 2 arguments
/// let closure_as_variable = |arg1:bool, arg2:u16|{
///     ...
/// };
/// let callback = callback!(closure_as_variable);
/// ```
/// 
/// ```compile_fail
/// // no arguments
/// let closure_as_variable = ||{
///     ...
/// };
/// let callback = callback!(closure_as_variable);
/// ```
/// 
/// - #### If you have closure variable with more or less than 1 argument, you can use on the the following direct methods:
///     - [Callback::new_with_args_0]
///     - [Callback::new_with_args_1]
///     - [Callback::new_with_args_2]
///     - [Callback::new_with_args_3]
///     - [Callback::new_with_args_4]
///     - [Callback::new_with_args_5]
///     - [Callback::new_with_args_6]
///     - [Callback::new_with_args_7]
///     - [Callback::new_with_args_8]
///
/// - #### A closure supplied directly to the [`callback`] macro can accept 0-8 arguments:
///     ```
///     let callback = callback!(|value:bool|{
///         //
///     });
///     ```
///     Output will be as follows:
///     ```
///     let callback = Callback::new_with_args_1(|value:bool|{
///         //
///     });
///     ```
/// - ##### Example of a closure with 2 arguments:
///     ```
///     let callback = callback!(|arg1:u16, value:bool|{
///         //
///     });
///     ```
///     Output will be as follows:
///     ```
///     let callback = Callback::new_with_args_2(|arg1:u16, value:bool|{
///        //
///     });
///     ```
/// 
pub use workflow_wasm_macros::callback;
