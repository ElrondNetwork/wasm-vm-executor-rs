use std::ffi::c_void;
use std::sync::Arc;
use std::borrow::Cow;

/// Represents a function pointer. It is mostly used in the
/// `typed_func` module within the `wrap` functions, to wrap imported
/// functions.
#[repr(transparent)]
pub struct FuncPtr(*mut c_void);

/// Const pointer to a `Func`.
#[derive(Debug, Clone)]
pub struct FuncPointer(*const FuncPtr);

impl FuncPointer {
    /// This needs to be unsafe because there is
    /// no way to check whether the passed function
    /// is valid and has the right signature.
    pub unsafe fn new(f: *const FuncPtr) -> Self {
        FuncPointer(f)
    }

    // pub(crate) fn inner(&self) -> *const FuncPtr {
    //     self.0
    // }
}

#[derive(Debug, Clone)]
pub struct Function {
    /// A pointer to a function.
    pub func: FuncPointer,
    /// The signature of the function.
    pub signature: Arc<FuncSig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncSig {
    // #[with(AsOwned)]
    params: Cow<'static, [Type]>,
    // #[with(AsOwned)]
    returns: Cow<'static, [Type]>,
}

impl FuncSig {
    /// Creates a new function signatures with the given parameter and return types.
    pub fn new<Params, Returns>(params: Params, returns: Returns) -> Self
    where
        Params: Into<Cow<'static, [Type]>>,
        Returns: Into<Cow<'static, [Type]>>,
    {
        Self {
            params: params.into(),
            returns: returns.into(),
        }
    }

    /// Parameter types.
    pub fn params(&self) -> &[Type] {
        &self.params
    }

    /// Return types.
    pub fn returns(&self) -> &[Type] {
        &self.returns
    }

    // /// Returns true if parameter types match the function signature.
    // pub fn check_param_value_types(&self, params: &[Value]) -> bool {
    //     self.params.len() == params.len()
    //         && self
    //             .params
    //             .iter()
    //             .zip(params.iter().map(|val| val.ty()))
    //             .all(|(t0, ref t1)| t0 == t1)
    // }
}

/// Represents a WebAssembly type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// The `i32` type.
    I32,
    /// The `i64` type.
    I64,
    /// The `f32` type.
    F32,
    /// The `f64` type.
    F64,
    /// The `v128` type.
    V128,
}
