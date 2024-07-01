use reader::types::Type;

pub fn get_cant_read_val_msg(typ: &(String, Type)) -> String {
    return format!(
        "Failed in reading expected type `{:?}` named: `{:?}`",
        typ.1, typ.0
    );
}
