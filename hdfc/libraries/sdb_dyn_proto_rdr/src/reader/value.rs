use compound_types::Cashflow;
use reader::types::Type;

#[derive(Debug)]
pub enum Value {
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    String(String),
    Cashflows(Vec<Cashflow>),
}

impl Value {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Value::I32(_) => Type::I32,
            Value::F32(_) => Type::F32,
            Value::I64(_) => Type::I64,
            Value::F64(_) => Type::F64,
            Value::String(_) => Type::String,
            Value::Cashflows(_) => Type::Cashflows,
        }
    }
}
