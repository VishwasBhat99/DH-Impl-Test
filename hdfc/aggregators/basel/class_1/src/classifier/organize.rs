use super::super::statics::Schema;

pub fn get_bucket_id(res_days: f64, bkt_def: &Vec<Schema>) -> String {
    let mut bucket_id: String = 0.to_string();
    for schema in bkt_def.iter() {
        if res_days >= schema.from_bkt && res_days <= schema.to_bkt {
            bucket_id = schema.id.to_string();
        } else {
            continue;
        }
    }
    bucket_id
}
