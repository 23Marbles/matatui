use std::marker::PhantomData;

pub mod mapping;
pub mod simplify;
pub mod solver;

pub struct SymbolicReprRef<'a> {
    _placeholder: PhantomData<&'a ()>,
}

impl SymbolicReprRef<'_> {
    pub fn to_owned(&self) -> SymbolicRepr {
        SymbolicRepr {}
    }
}

pub struct SymbolicRepr {}

impl SymbolicRepr {
    pub fn as_ref(&self) -> SymbolicReprRef<'_> {
        SymbolicReprRef {
            _placeholder: PhantomData,
        }
    }
}
