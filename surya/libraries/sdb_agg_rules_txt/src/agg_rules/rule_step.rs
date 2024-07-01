use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::Reader;
use std::str::Split;

#[derive(Debug)]
pub(crate) struct RuleStep {
    pub(crate) field_name: String,
    pub(crate) comparator: Comparator,
}

impl RuleStep {
    pub fn does_acc_abide(&self, account: &String, reader: &Reader) -> bool {
        let fields: Vec<&str> = account.split('|').collect();
        match &self.comparator {
            // TODO: Remove `expect`s
            // TODO: Is there is a way we can cut out this boiler-plate?
            Comparator::EQ(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v == &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v == &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v == &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v == &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v == field_value_str;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::NE(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v != &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v != &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v != &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v != &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v != field_value_str;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::LT(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v > &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v > &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v > &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v > &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v > &field_value_str.to_string();
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::LTEQ(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v >= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v >= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v >= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v >= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v >= &field_value_str.to_string();
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::GT(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v < &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v < &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v < &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v < &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v < &field_value_str.to_string();
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::GTEQ(expected_val) => match expected_val {
                Value::I32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v <= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F32(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f32 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v <= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::I64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: i64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v <= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::F64(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        let field_value: f64 = field_value_str
                            .parse()
                            .expect("Cannot parse string as i32.");
                        return e_v <= &field_value;
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
                Value::String(e_v) => match reader.get_field_pos(&self.field_name) {
                    Some(val) => {
                        let field_value_str = fields[val - 1];
                        return e_v <= &field_value_str.to_string();
                    }
                    None => {
                        panic!("Cannot read {} field in input.", self.field_name);
                    }
                },
            },
            Comparator::BTWN(expected_vals) => {
                let expected_type = self.comparator.get_type();

                match expected_type {
                    Type::I32 => match (&expected_vals.0, &expected_vals.1) {
                        (Value::I32(v1), Value::I32(v2)) => {
                            match reader.get_field_pos(&self.field_name) {
                                Some(val) => {
                                    let field_value_str = fields[val - 1];
                                    let field_value: i32 = field_value_str
                                        .parse()
                                        .expect("Cannot parse string as i32.");
                                    return v1 < &field_value && &field_value < v2;
                                }
                                None => {
                                    panic!("Cannot read {} field in input.", self.field_name);
                                }
                            }
                        }
                        _ => panic!(""),
                    },
                    Type::F32 => match (&expected_vals.0, &expected_vals.1) {
                        (Value::F32(v1), Value::F32(v2)) => {
                            match reader.get_field_pos(&self.field_name) {
                                Some(val) => {
                                    let field_value_str = fields[val - 1];
                                    let field_value: f32 = field_value_str
                                        .parse()
                                        .expect("Cannot parse string as i32.");
                                    return v1 < &field_value && &field_value < v2;
                                }
                                None => {
                                    panic!("Cannot read {} field in input.", self.field_name);
                                }
                            }
                        }
                        _ => panic!(""),
                    },
                    Type::I64 => match (&expected_vals.0, &expected_vals.1) {
                        (Value::I64(v1), Value::I64(v2)) => {
                            match reader.get_field_pos(&self.field_name) {
                                Some(val) => {
                                    let field_value_str = fields[val - 1];
                                    let field_value: i64 = field_value_str
                                        .parse()
                                        .expect("Cannot parse string as i32.");
                                    return v1 < &field_value && &field_value < v2;
                                }
                                None => {
                                    panic!("Cannot read {} field in input.", self.field_name);
                                }
                            }
                        }
                        _ => panic!(""),
                    },
                    Type::F64 => match (&expected_vals.0, &expected_vals.1) {
                        (Value::F64(v1), Value::F64(v2)) => {
                            match reader.get_field_pos(&self.field_name) {
                                Some(val) => {
                                    let field_value_str = fields[val - 1];
                                    let field_value: f64 = field_value_str
                                        .parse()
                                        .expect("Cannot parse string as i32.");
                                    return v1 < &field_value && &field_value < v2;
                                }
                                None => {
                                    panic!("Cannot read {} field in input.", self.field_name);
                                }
                            }
                        }
                        _ => panic!(""),
                    },
                    Type::String => match (&expected_vals.0, &expected_vals.1) {
                        (Value::String(v1), Value::String(v2)) => {
                            match reader.get_field_pos(&self.field_name) {
                                Some(val) => {
                                    let field_value_str = fields[val - 1].to_string();
                                    return v1 < &field_value_str && &field_value_str < v2;
                                }
                                None => {
                                    panic!("Cannot read {} field in input.", self.field_name);
                                }
                            }
                        }
                        _ => panic!(""),
                    },
                }
            }
            Comparator::IN(expected_val) => match expected_val {
                Value::String(e_v) => {
                    let acc_val;
                    match reader.get_field_pos(&self.field_name) {
                        Some(val) => {
                            acc_val = fields[val - 1];
                        }
                        None => {
                            panic!("Cannot read {} field in input.", self.field_name);
                        }
                    }
                    let input_val = ",".to_string() + &acc_val.to_owned() + ",";
                    let rule_val = ",".to_string() + &e_v.to_owned().to_string() + &",".to_string();
                    if rule_val.contains(&input_val) {
                        return true;
                    }
                    return false;
                }
                _ => panic!("Unexpected error!!"),
            },
            Comparator::NTIN(expected_val) => match expected_val {
                Value::String(e_v) => {
                    let acc_val;
                    match reader.get_field_pos(&self.field_name) {
                        Some(val) => {
                            acc_val = fields[val - 1];
                        }
                        None => {
                            panic!("Cannot read {} field in input.", self.field_name);
                        }
                    }

                    let input_val = ",".to_string() + &acc_val.to_owned() + ",";
                    let rule_val = ",".to_string() + &e_v.to_owned().to_string() + &",".to_string();
                    if !rule_val.contains(&input_val) {
                        return true;
                    }
                    return false;
                }
                _ => panic!("Unexpected error!!"),
            },
        }
    }
}

#[derive(Debug)]
pub enum Comparator {
    EQ(Value),
    LT(Value),
    GT(Value),
    NE(Value),
    GTEQ(Value),
    LTEQ(Value),
    BTWN((Value, Value)), // TODO: Ensure we have the same types on both sides
    IN(Value),
    NTIN(Value),
}

impl Comparator {
    pub fn new_from_iter(iter: &mut Split<char>, typ: reader::types::Type) -> Self {
        let comparator_str = iter.next().expect("Comparator not present");
        match comparator_str {
            "EQ" => {
                return Comparator::EQ(Comparator::get_next_as_val(iter, typ));
            }
            "LT" => {
                return Comparator::LT(Comparator::get_next_as_val(iter, typ));
            }
            "GT" => {
                return Comparator::GT(Comparator::get_next_as_val(iter, typ));
            }
            "NE" => {
                return Comparator::NE(Comparator::get_next_as_val(iter, typ));
            }
            "GTEQ" => {
                return Comparator::GTEQ(Comparator::get_next_as_val(iter, typ));
            }
            "LTEQ" => {
                return Comparator::LTEQ(Comparator::get_next_as_val(iter, typ));
            }
            "BTWN" => {
                return Comparator::BTWN((
                    Comparator::get_next_as_val(iter, typ),
                    Comparator::get_next_as_val(iter, typ),
                ));
            }
            "IN" => {
                return Comparator::IN(Value::String(
                    iter.next().expect("Comparator not present").to_string(),
                ));
            }
            "NTIN" => {
                return Comparator::NTIN(Value::String(
                    iter.next().expect("Comparator not present").to_string(),
                ));
            }
            _ => {
                panic!("Unexpected comparator");
            }
        }
    }

    fn get_next_as_val(iter: &mut Split<char>, typ: reader::types::Type) -> Value {
        let val_str = iter.next().expect("Comparator not present");
        match typ {
            reader::types::Type::I32 => {
                return Value::I32(val_str.parse().expect("Type mismatch"));
            }
            reader::types::Type::F32 => {
                return Value::F32(val_str.parse().expect("Type mismatch"));
            }
            reader::types::Type::I64 => {
                return Value::I64(val_str.parse().expect("Type mismatch"));
            }
            reader::types::Type::F64 => {
                return Value::F64(val_str.parse().expect("Type mismatch"));
            }
            reader::types::Type::String => {
                return Value::String(val_str.to_string());
            }
            reader::types::Type::Cashflows => {
                panic!("Rules against the cashflow type are not supported");
            }
        }
    }

    fn get_type(&self) -> Type {
        match self {
            Comparator::EQ(v) => {
                return v.get_type();
            }
            Comparator::LT(v) => {
                return v.get_type();
            }
            Comparator::GT(v) => {
                return v.get_type();
            }
            Comparator::NE(v) => {
                return v.get_type();
            }
            Comparator::GTEQ(v) => {
                return v.get_type();
            }
            Comparator::LTEQ(v) => {
                return v.get_type();
            }
            Comparator::BTWN(vals) => {
                let val1 = vals.0.get_type();
                let val2 = vals.1.get_type();
                if val1 != val2 {
                    // TODO: Provide more information.
                    panic!("The lower and upper bound values of a `BTWN` comparator are not of the same type.")
                }
                return val1;
            }
            Comparator::IN(v) => {
                return v.get_type();
            }
            Comparator::NTIN(v) => {
                return v.get_type();
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    String(String),
}

impl Value {
    fn get_type(&self) -> Type {
        match self {
            Value::I32(_) => return Type::I32,
            Value::I64(_) => return Type::I64,
            Value::F32(_) => return Type::F32,
            Value::F64(_) => return Type::F64,
            Value::String(_) => return Type::String,
        }
    }
}

#[derive(PartialEq)]
pub enum Type {
    I32,
    F32,
    I64,
    F64,
    String,
}
