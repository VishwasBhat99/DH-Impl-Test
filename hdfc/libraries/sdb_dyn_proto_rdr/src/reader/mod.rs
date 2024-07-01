use compound_types::cashflow::Cashflow;
use protobuf::error::WireError;
use protobuf::rt::read_repeated_message_into;
use protobuf::CodedInputStream;
use protobuf::ProtobufError;
use protobuf::RepeatedField;
use reader::account_with_cfs::AccountWithCFs;
use reader::error_msg_utils::get_cant_read_val_msg;
use reader::metadata_parser::get_metadata_from_path;
use reader::types::Type;
use reader::value::Value;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub mod account_with_cfs;
mod error_msg_utils;
mod metadata_parser;
pub mod types;
pub(crate) mod value;

#[macro_use]
mod macros;

pub struct Reader {
    fields_key_pos: HashMap<String, usize>,
    fields_pos_desc: HashMap<u32, (String, Type)>,
    ord_field_keys: Vec<String>,
    rdr: BufReader<File>,
}

impl Reader {
    pub fn new_at_path(schema_path: &str, data_file_path: &str) -> Reader {
        let (fields_key_pos, fields_pos_desc) = get_metadata_from_path(schema_path);

        // Get field names in the order they are supposed to appear.
        //
        // TODO: there is a bug here: If a user writes the proto file by skipping the number of a field
        // ie: (1,2,4,5...)
        // we will attempt to lookup value at position 5 and crash with index out of bounds.
        //
        // To solve this problem, we will create a struct that encapsulates the proto index
        // and the sequential index.
        // Do this in the map function.
        let mut fields_key_pos_vec: Vec<(String, usize)> = fields_key_pos
            .iter()
            .map(|(name, pos)| (name.clone(), *pos))
            .collect();
        fields_key_pos_vec.sort_by(|curr, next| curr.1.partial_cmp(&next.1).unwrap());
        let ord_field_keys = fields_key_pos_vec
            .drain(..)
            .map(|(name, _pos)| name)
            .collect();

        let rdr = new_buf_rdr(&data_file_path).expect(&format!(
            "Could not read file at path: `{}`",
            data_file_path
        ));

        return Reader {
            fields_key_pos,
            fields_pos_desc,
            ord_field_keys,
            rdr,
        };
    }

    pub fn get_field_type(&self, field: &String) -> Option<Type> {
        let index = *self.fields_key_pos.get(field)? as u32;
        return Some(self.fields_pos_desc.get(&index)?.1);
    }

    pub fn get_field_pos(&self, field: &String) -> Option<usize> {
        match self.fields_key_pos.get(field) {
            Some(val) => Some(*val),
            None => None,
        }
    }
}

impl Reader {
    pub fn iter<'a>(&'a mut self) -> impl Iterator<Item = AccountWithCFs<'a>> {
        let x = _Reader {
            ordered_field_keys: &self.ord_field_keys,
            field_keys: self.fields_key_pos.clone(), // TODO: Reconsider Clone
            fields_pos_desc: self.fields_pos_desc.clone(),
            rcrd_buf: Vec::new(),
            input_strm: CodedInputStream::new(&mut self.rdr),
            max_encountered_cfs_len: 0,
        };

        return x;
    }
}

struct _Reader<'a> {
    ordered_field_keys: &'a Vec<String>,
    field_keys: HashMap<String, usize>,
    fields_pos_desc: HashMap<u32, (String, Type)>,
    rcrd_buf: Vec<u8>,
    input_strm: CodedInputStream<'a>,
    max_encountered_cfs_len: usize,
}

impl<'a> Iterator for _Reader<'a> {
    type Item = AccountWithCFs<'a>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let mut map = HashMap::new();

        let rcrd_size = match self.input_strm.read_raw_varint32() {
            Ok(s) => s,
            Err(e) => match e {
                ProtobufError::WireError(wire_err) => match wire_err {
                    WireError::UnexpectedEof => {
                        return None;
                    }
                    _ => panic!("Error reading size of record: {:?}", wire_err),
                },
                _ => panic!("Error reading size of record: {}", e),
            },
        };
        if rcrd_size == 0 {
            return None;
        }
        self.rcrd_buf.clear();
        self.input_strm
            .read_raw_bytes_into(rcrd_size, &mut self.rcrd_buf)
            .expect(&format!(
                "Could not read record of size {} into buffer",
                rcrd_size
            ));

        self.fill_acc_w_cfs_map(&mut map);

        let mut acc = AccountWithCFs::new();
        acc.map = map;
        acc.rec_bytes = self.rcrd_buf.to_vec();
        return Some(acc);
    }
}

impl<'a> _Reader<'a> {
    fn fill_acc_w_cfs_map(&mut self, map: &mut HashMap<&'a String, Value>) {
        let mut stream = CodedInputStream::from_bytes(&self.rcrd_buf);

        // Artifcats for reading repeated cashflows
        let cfs_vec = Vec::with_capacity(self.max_encountered_cfs_len);
        let mut repeated_cfs: RepeatedField<Cashflow> = RepeatedField::from_vec(cfs_vec);
        let mut cfs_field_name: Option<&String> = None;

        while !stream.eof().unwrap() {
            let (field_no, wire_type) = stream.read_tag_unpack().expect("Unable to read proto tag");

            let value = self
                .fields_pos_desc
                .get(&field_no)
                .expect("Unexpected tag number in proto record data");
            let position = *val_or_crash_with!(self.field_keys.get(&value.0), {
                format!("Position of field `{:?}` not known", value)
            });

            // TODO: This will out of bound because proto's field no. is not the same as the index number.
            // Read `fn new_at_path` for more.
            let field_name = &self.ordered_field_keys[position - 1];

            match &value.1 {
                Type::I32 => {
                    map.insert(
                        field_name,
                        Value::I32(res_or_crash_with!(stream.read_int32(), {
                            get_cant_read_val_msg(value)
                        })),
                    );
                }
                Type::F32 => {
                    map.insert(
                        field_name,
                        Value::F32(res_or_crash_with!(stream.read_float(), {
                            get_cant_read_val_msg(value)
                        })),
                    );
                }
                Type::I64 => {
                    map.insert(
                        field_name,
                        Value::I64(res_or_crash_with!(stream.read_int64(), {
                            get_cant_read_val_msg(value)
                        })),
                    );
                }
                Type::F64 => {
                    map.insert(
                        field_name,
                        Value::F64(res_or_crash_with!(stream.read_double(), {
                            get_cant_read_val_msg(value)
                        })),
                    );
                }
                Type::String => {
                    map.insert(
                        field_name,
                        Value::String(res_or_crash_with!(stream.read_string(), {
                            get_cant_read_val_msg(value)
                        })),
                    );
                }
                Type::Cashflows => {
                    res_or_crash_with!(
                        read_repeated_message_into(wire_type, &mut stream, &mut repeated_cfs),
                        get_cant_read_val_msg(value)
                    );

                    // At this point, I can't reason about why this situation may occur:
                    // We were told that a repeated field value exists but the `RepeatedField` is
                    // still empty?
                    //
                    // We've put a check against inserting empty values for the cashflows vec, for now.
                    if repeated_cfs.len() > 0 {
                        if cfs_field_name == None {
                            cfs_field_name = Some(field_name);
                        }
                    }
                }
            }
        }

        // Optimisation for reducing number of `malloc`s for future accounts
        if repeated_cfs.len() > self.max_encountered_cfs_len {
            self.max_encountered_cfs_len = repeated_cfs.len();
        }

        if cfs_field_name.is_some() {
            map.insert(
                cfs_field_name.unwrap(),
                Value::Cashflows(repeated_cfs.into()),
            );
        }
    }
}
