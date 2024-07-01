pub use compound_types::cashflow::Cashflow;
use reader::types::Type;
use reader::value::Value;
use reader::Reader;
use std::collections::HashMap;

#[derive(Debug)]
/// An account in the `.cf` file.
pub struct AccountWithCFs<'a> {
    pub map: HashMap<&'a String, Value>,
    pub rec_bytes: Vec<u8>,
}

impl<'a> AccountWithCFs<'a> {
    pub fn new() -> AccountWithCFs<'a> {
        AccountWithCFs {
            map: HashMap::new(),
            rec_bytes: Vec::new(),
        }
    }
}

impl<'a> AccountWithCFs<'a> {
    /// Returns an `i32` for the key you provide.
    pub fn get_i32_for_key(&self, key: &String) -> Result<i32, ValueRetrievalError> {
        let value = match self.get_value(key) {
            Ok(v) => v,
            Err(e) => match e {
                ValueRetrievalError::ValueNotPresent => {
                    return Ok(0);
                }
                ValueRetrievalError::TypeMismatch(_) => {
                    panic!(
                        "Unhandled error fetching value for key `{:?}`: {:?}",
                        key, e
                    );
                }
            },
        };

        match value {
            Value::I32(v) => Ok(*v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    /// Returns an `f32` for the key you provide.
    pub fn get_f32_for_key(&self, key: &String) -> Result<f32, ValueRetrievalError> {
        let value = match self.get_value(key) {
            Ok(v) => v,
            Err(e) => match e {
                ValueRetrievalError::ValueNotPresent => {
                    return Ok(0.0);
                }
                ValueRetrievalError::TypeMismatch(_) => {
                    panic!(
                        "Unhandled error fetching value for key `{:?}`: {:?}",
                        key, e
                    );
                }
            },
        };

        match value {
            Value::F32(v) => Ok(*v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    /// Returns an `i64` for the key you provide.
    pub fn get_i64_for_key(&self, key: &String) -> Result<i64, ValueRetrievalError> {
        let value = match self.get_value(key) {
            Ok(v) => v,
            Err(e) => match e {
                ValueRetrievalError::ValueNotPresent => {
                    return Ok(0);
                }
                ValueRetrievalError::TypeMismatch(_) => {
                    panic!(
                        "Unhandled error fetching value for key `{:?}`: {:?}",
                        key, e
                    );
                }
            },
        };

        match value {
            Value::I64(v) => Ok(*v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    /// Returns an `f64` for the key you provide.
    pub fn get_f64_for_key(&self, key: &String) -> Result<f64, ValueRetrievalError> {
        let value = match self.get_value(key) {
            Ok(v) => v,
            Err(e) => match e {
                ValueRetrievalError::ValueNotPresent => {
                    return Ok(0.0);
                }
                ValueRetrievalError::TypeMismatch(_) => {
                    panic!(
                        "Unhandled error fetching value for key `{:?}`: {:?}",
                        key, e
                    );
                }
            },
        };

        match value {
            Value::F64(v) => Ok(*v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    /// Returns a string for the key you provide.
    /// The string returned to you is a borrowed value of the underlying string.
    pub fn get_string_for_key(&self, key: &String) -> Result<&String, ValueRetrievalError> {
        let value = self.get_value(key)?;

        match value {
            Value::String(v) => Ok(v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    /// Returns a `vec` of cashflows in this account.
    /// Once you've removed cfs for this account, calling this function again will error with `ValueRetrievalError::ValueNotPresent`.
    pub fn remove_cfs_for_key(
        &mut self,
        key: &String,
    ) -> Result<Vec<Cashflow>, ValueRetrievalError> {
        let value = self.remove_value(key)?;

        match value {
            Value::Cashflows(v) => Ok(v),
            _ => Err(ValueRetrievalError::TypeMismatch(value.get_type())),
        }
    }

    fn get_value(&self, k: &String) -> Result<&Value, ValueRetrievalError> {
        if let Some(v) = self.map.get(k) {
            Ok(v)
        } else {
            Err(ValueRetrievalError::ValueNotPresent)
        }
    }

    fn remove_value(&mut self, k: &String) -> Result<Value, ValueRetrievalError> {
        if let Some(v) = self.map.remove(k) {
            Ok(v)
        } else {
            Err(ValueRetrievalError::ValueNotPresent)
        }
    }
}

#[derive(Debug)]
/// A list of errors that can be produced when asking for a property.
pub enum ValueRetrievalError {
    TypeMismatch(Type),
    ValueNotPresent, // TODO: include property name
}

pub fn get_field_value(
    account: &AccountWithCFs,
    method_reader: &Reader,
    field_name: String,
) -> Result<String, String> {
    match method_reader
        .get_field_type(&field_name)
        .unwrap_or(Type::String)
    {
        Type::I32 => Ok(account
            .get_i32_for_key(&field_name)
            .unwrap_or(0)
            .to_string()),
        Type::I64 => Ok(account
            .get_i64_for_key(&field_name)
            .unwrap_or(0)
            .to_string()),
        Type::F32 => Ok(account
            .get_f32_for_key(&field_name)
            .unwrap_or(0.0)
            .to_string()),
        Type::F64 => Ok(account
            .get_f64_for_key(&field_name)
            .unwrap_or(0.0)
            .to_string()),
        Type::String => Ok(account
            .get_string_for_key(&field_name)
            .unwrap_or(&"".to_string())
            .to_string()),
        _ => Err("Invalid field type found".to_string()),
    }
}
