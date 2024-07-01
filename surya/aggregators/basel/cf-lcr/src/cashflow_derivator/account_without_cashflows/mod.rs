#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

#[derive(PartialEq,Clone,Default)]
pub struct OutputAccount {
    // message fields
    pub file_id: ::std::string::String,
    pub ca: f64,
    pub sa: f64,
    pub td_wd: f64,
    pub td_nwd: f64,
    pub rd: f64,
    pub tot_stable: f64,
    pub tot_less_stable: f64,
    pub ca_stable: f64,
    pub ca_less_stable: f64,
    pub sa_stable: f64,
    pub sa_less_stable: f64,
    pub casa_stable: f64,
    pub casa_less_stable: f64,
    pub stable_b1: f64,
    pub stable_b2: f64,
    pub stable_b3: f64,
    pub less_stable_b1: f64,
    pub less_stable_b2: f64,
    pub less_stable_b3: f64,
    pub nwd_b1: f64,
    pub nwd_b2: f64,
    pub nwd_b3: f64,
    pub currency: ::std::string::String,
    pub nwd_stable_b1: f64,
    pub nwd_stable_b2: f64,
    pub nwd_stable_b3: f64,
    pub nwd_less_stable_b1: f64,
    pub nwd_less_stable_b2: f64,
    pub nwd_less_stable_b3: f64,
    pub ca_wt_int_rate: f64,
    pub sa_wt_int_rate: f64,
    pub td_wd_wt_int_rate: f64,
    pub td_nwd_wt_int_rate: f64,
    pub rd_wt_int_rate: f64,
    pub lien_amt: f64,
    pub bal_before_lien: f64,
    pub bal_after_lien: f64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OutputAccount {
    fn default() -> &'a OutputAccount {
        <OutputAccount as ::protobuf::Message>::default_instance()
    }
}

impl OutputAccount {
    pub fn new() -> OutputAccount {
        ::std::default::Default::default()
    }

    // string file_id = 1;


    pub fn get_file_id(&self) -> &str {
        &self.file_id
    }
    pub fn clear_file_id(&mut self) {
        self.file_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_id(&mut self, v: ::std::string::String) {
        self.file_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_id(&mut self) -> &mut ::std::string::String {
        &mut self.file_id
    }

    // Take field
    pub fn take_file_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.file_id, ::std::string::String::new())
    }

    // double ca = 2;


    pub fn get_ca(&self) -> f64 {
        self.ca
    }
    pub fn clear_ca(&mut self) {
        self.ca = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ca(&mut self, v: f64) {
        self.ca = v;
    }

    // double sa = 3;


    pub fn get_sa(&self) -> f64 {
        self.sa
    }
    pub fn clear_sa(&mut self) {
        self.sa = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sa(&mut self, v: f64) {
        self.sa = v;
    }

    // double td_wd = 4;


    pub fn get_td_wd(&self) -> f64 {
        self.td_wd
    }
    pub fn clear_td_wd(&mut self) {
        self.td_wd = 0.;
    }

    // Param is passed by value, moved
    pub fn set_td_wd(&mut self, v: f64) {
        self.td_wd = v;
    }

    // double td_nwd = 5;


    pub fn get_td_nwd(&self) -> f64 {
        self.td_nwd
    }
    pub fn clear_td_nwd(&mut self) {
        self.td_nwd = 0.;
    }

    // Param is passed by value, moved
    pub fn set_td_nwd(&mut self, v: f64) {
        self.td_nwd = v;
    }

    // double rd = 6;


    pub fn get_rd(&self) -> f64 {
        self.rd
    }
    pub fn clear_rd(&mut self) {
        self.rd = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rd(&mut self, v: f64) {
        self.rd = v;
    }

    // double tot_stable = 7;


    pub fn get_tot_stable(&self) -> f64 {
        self.tot_stable
    }
    pub fn clear_tot_stable(&mut self) {
        self.tot_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tot_stable(&mut self, v: f64) {
        self.tot_stable = v;
    }

    // double tot_less_stable = 8;


    pub fn get_tot_less_stable(&self) -> f64 {
        self.tot_less_stable
    }
    pub fn clear_tot_less_stable(&mut self) {
        self.tot_less_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_tot_less_stable(&mut self, v: f64) {
        self.tot_less_stable = v;
    }

    // double ca_stable = 9;


    pub fn get_ca_stable(&self) -> f64 {
        self.ca_stable
    }
    pub fn clear_ca_stable(&mut self) {
        self.ca_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ca_stable(&mut self, v: f64) {
        self.ca_stable = v;
    }

    // double ca_less_stable = 10;


    pub fn get_ca_less_stable(&self) -> f64 {
        self.ca_less_stable
    }
    pub fn clear_ca_less_stable(&mut self) {
        self.ca_less_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ca_less_stable(&mut self, v: f64) {
        self.ca_less_stable = v;
    }

    // double sa_stable = 11;


    pub fn get_sa_stable(&self) -> f64 {
        self.sa_stable
    }
    pub fn clear_sa_stable(&mut self) {
        self.sa_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sa_stable(&mut self, v: f64) {
        self.sa_stable = v;
    }

    // double sa_less_stable = 12;


    pub fn get_sa_less_stable(&self) -> f64 {
        self.sa_less_stable
    }
    pub fn clear_sa_less_stable(&mut self) {
        self.sa_less_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sa_less_stable(&mut self, v: f64) {
        self.sa_less_stable = v;
    }

    // double casa_stable = 13;


    pub fn get_casa_stable(&self) -> f64 {
        self.casa_stable
    }
    pub fn clear_casa_stable(&mut self) {
        self.casa_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_casa_stable(&mut self, v: f64) {
        self.casa_stable = v;
    }

    // double casa_less_stable = 14;


    pub fn get_casa_less_stable(&self) -> f64 {
        self.casa_less_stable
    }
    pub fn clear_casa_less_stable(&mut self) {
        self.casa_less_stable = 0.;
    }

    // Param is passed by value, moved
    pub fn set_casa_less_stable(&mut self, v: f64) {
        self.casa_less_stable = v;
    }

    // double stable_b1 = 15;


    pub fn get_stable_b1(&self) -> f64 {
        self.stable_b1
    }
    pub fn clear_stable_b1(&mut self) {
        self.stable_b1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_stable_b1(&mut self, v: f64) {
        self.stable_b1 = v;
    }

    // double stable_b2 = 16;


    pub fn get_stable_b2(&self) -> f64 {
        self.stable_b2
    }
    pub fn clear_stable_b2(&mut self) {
        self.stable_b2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_stable_b2(&mut self, v: f64) {
        self.stable_b2 = v;
    }

    // double stable_b3 = 17;


    pub fn get_stable_b3(&self) -> f64 {
        self.stable_b3
    }
    pub fn clear_stable_b3(&mut self) {
        self.stable_b3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_stable_b3(&mut self, v: f64) {
        self.stable_b3 = v;
    }

    // double less_stable_b1 = 18;


    pub fn get_less_stable_b1(&self) -> f64 {
        self.less_stable_b1
    }
    pub fn clear_less_stable_b1(&mut self) {
        self.less_stable_b1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_less_stable_b1(&mut self, v: f64) {
        self.less_stable_b1 = v;
    }

    // double less_stable_b2 = 19;


    pub fn get_less_stable_b2(&self) -> f64 {
        self.less_stable_b2
    }
    pub fn clear_less_stable_b2(&mut self) {
        self.less_stable_b2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_less_stable_b2(&mut self, v: f64) {
        self.less_stable_b2 = v;
    }

    // double less_stable_b3 = 20;


    pub fn get_less_stable_b3(&self) -> f64 {
        self.less_stable_b3
    }
    pub fn clear_less_stable_b3(&mut self) {
        self.less_stable_b3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_less_stable_b3(&mut self, v: f64) {
        self.less_stable_b3 = v;
    }

    // double nwd_b1 = 21;


    pub fn get_nwd_b1(&self) -> f64 {
        self.nwd_b1
    }
    pub fn clear_nwd_b1(&mut self) {
        self.nwd_b1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_b1(&mut self, v: f64) {
        self.nwd_b1 = v;
    }

    // double nwd_b2 = 22;


    pub fn get_nwd_b2(&self) -> f64 {
        self.nwd_b2
    }
    pub fn clear_nwd_b2(&mut self) {
        self.nwd_b2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_b2(&mut self, v: f64) {
        self.nwd_b2 = v;
    }

    // double nwd_b3 = 23;


    pub fn get_nwd_b3(&self) -> f64 {
        self.nwd_b3
    }
    pub fn clear_nwd_b3(&mut self) {
        self.nwd_b3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_b3(&mut self, v: f64) {
        self.nwd_b3 = v;
    }

    // string currency = 24;


    pub fn get_currency(&self) -> &str {
        &self.currency
    }
    pub fn clear_currency(&mut self) {
        self.currency.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency(&mut self) -> &mut ::std::string::String {
        &mut self.currency
    }

    // Take field
    pub fn take_currency(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency, ::std::string::String::new())
    }

    // double nwd_stable_b1 = 25;


    pub fn get_nwd_stable_b1(&self) -> f64 {
        self.nwd_stable_b1
    }
    pub fn clear_nwd_stable_b1(&mut self) {
        self.nwd_stable_b1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_stable_b1(&mut self, v: f64) {
        self.nwd_stable_b1 = v;
    }

    // double nwd_stable_b2 = 26;


    pub fn get_nwd_stable_b2(&self) -> f64 {
        self.nwd_stable_b2
    }
    pub fn clear_nwd_stable_b2(&mut self) {
        self.nwd_stable_b2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_stable_b2(&mut self, v: f64) {
        self.nwd_stable_b2 = v;
    }

    // double nwd_stable_b3 = 27;


    pub fn get_nwd_stable_b3(&self) -> f64 {
        self.nwd_stable_b3
    }
    pub fn clear_nwd_stable_b3(&mut self) {
        self.nwd_stable_b3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_stable_b3(&mut self, v: f64) {
        self.nwd_stable_b3 = v;
    }

    // double nwd_less_stable_b1 = 28;


    pub fn get_nwd_less_stable_b1(&self) -> f64 {
        self.nwd_less_stable_b1
    }
    pub fn clear_nwd_less_stable_b1(&mut self) {
        self.nwd_less_stable_b1 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_less_stable_b1(&mut self, v: f64) {
        self.nwd_less_stable_b1 = v;
    }

    // double nwd_less_stable_b2 = 29;


    pub fn get_nwd_less_stable_b2(&self) -> f64 {
        self.nwd_less_stable_b2
    }
    pub fn clear_nwd_less_stable_b2(&mut self) {
        self.nwd_less_stable_b2 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_less_stable_b2(&mut self, v: f64) {
        self.nwd_less_stable_b2 = v;
    }

    // double nwd_less_stable_b3 = 30;


    pub fn get_nwd_less_stable_b3(&self) -> f64 {
        self.nwd_less_stable_b3
    }
    pub fn clear_nwd_less_stable_b3(&mut self) {
        self.nwd_less_stable_b3 = 0.;
    }

    // Param is passed by value, moved
    pub fn set_nwd_less_stable_b3(&mut self, v: f64) {
        self.nwd_less_stable_b3 = v;
    }

    // double ca_wt_int_rate = 31;


    pub fn get_ca_wt_int_rate(&self) -> f64 {
        self.ca_wt_int_rate
    }
    pub fn clear_ca_wt_int_rate(&mut self) {
        self.ca_wt_int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ca_wt_int_rate(&mut self, v: f64) {
        self.ca_wt_int_rate = v;
    }

    // double sa_wt_int_rate = 32;


    pub fn get_sa_wt_int_rate(&self) -> f64 {
        self.sa_wt_int_rate
    }
    pub fn clear_sa_wt_int_rate(&mut self) {
        self.sa_wt_int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_sa_wt_int_rate(&mut self, v: f64) {
        self.sa_wt_int_rate = v;
    }

    // double td_wd_wt_int_rate = 33;


    pub fn get_td_wd_wt_int_rate(&self) -> f64 {
        self.td_wd_wt_int_rate
    }
    pub fn clear_td_wd_wt_int_rate(&mut self) {
        self.td_wd_wt_int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_td_wd_wt_int_rate(&mut self, v: f64) {
        self.td_wd_wt_int_rate = v;
    }

    // double td_nwd_wt_int_rate = 34;


    pub fn get_td_nwd_wt_int_rate(&self) -> f64 {
        self.td_nwd_wt_int_rate
    }
    pub fn clear_td_nwd_wt_int_rate(&mut self) {
        self.td_nwd_wt_int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_td_nwd_wt_int_rate(&mut self, v: f64) {
        self.td_nwd_wt_int_rate = v;
    }

    // double rd_wt_int_rate = 35;


    pub fn get_rd_wt_int_rate(&self) -> f64 {
        self.rd_wt_int_rate
    }
    pub fn clear_rd_wt_int_rate(&mut self) {
        self.rd_wt_int_rate = 0.;
    }

    // Param is passed by value, moved
    pub fn set_rd_wt_int_rate(&mut self, v: f64) {
        self.rd_wt_int_rate = v;
    }

    // double lien_amt = 36;


    pub fn get_lien_amt(&self) -> f64 {
        self.lien_amt
    }
    pub fn clear_lien_amt(&mut self) {
        self.lien_amt = 0.;
    }

    // Param is passed by value, moved
    pub fn set_lien_amt(&mut self, v: f64) {
        self.lien_amt = v;
    }

    // double bal_before_lien = 37;


    pub fn get_bal_before_lien(&self) -> f64 {
        self.bal_before_lien
    }
    pub fn clear_bal_before_lien(&mut self) {
        self.bal_before_lien = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bal_before_lien(&mut self, v: f64) {
        self.bal_before_lien = v;
    }

    // double bal_after_lien = 38;


    pub fn get_bal_after_lien(&self) -> f64 {
        self.bal_after_lien
    }
    pub fn clear_bal_after_lien(&mut self) {
        self.bal_after_lien = 0.;
    }

    // Param is passed by value, moved
    pub fn set_bal_after_lien(&mut self, v: f64) {
        self.bal_after_lien = v;
    }
}

impl ::protobuf::Message for OutputAccount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.file_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ca = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sa = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.td_wd = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.td_nwd = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.rd = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_stable = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.tot_less_stable = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ca_stable = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ca_less_stable = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sa_stable = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sa_less_stable = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.casa_stable = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.casa_less_stable = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.stable_b1 = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.stable_b2 = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.stable_b3 = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.less_stable_b1 = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.less_stable_b2 = tmp;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.less_stable_b3 = tmp;
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_b1 = tmp;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_b2 = tmp;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_b3 = tmp;
                },
                24 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency)?;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_stable_b1 = tmp;
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_stable_b2 = tmp;
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_stable_b3 = tmp;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_less_stable_b1 = tmp;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_less_stable_b2 = tmp;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.nwd_less_stable_b3 = tmp;
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ca_wt_int_rate = tmp;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.sa_wt_int_rate = tmp;
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.td_wd_wt_int_rate = tmp;
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.td_nwd_wt_int_rate = tmp;
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.rd_wt_int_rate = tmp;
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.lien_amt = tmp;
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.bal_before_lien = tmp;
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.bal_after_lien = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.file_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file_id);
        }
        if self.ca != 0. {
            my_size += 9;
        }
        if self.sa != 0. {
            my_size += 9;
        }
        if self.td_wd != 0. {
            my_size += 9;
        }
        if self.td_nwd != 0. {
            my_size += 9;
        }
        if self.rd != 0. {
            my_size += 9;
        }
        if self.tot_stable != 0. {
            my_size += 9;
        }
        if self.tot_less_stable != 0. {
            my_size += 9;
        }
        if self.ca_stable != 0. {
            my_size += 9;
        }
        if self.ca_less_stable != 0. {
            my_size += 9;
        }
        if self.sa_stable != 0. {
            my_size += 9;
        }
        if self.sa_less_stable != 0. {
            my_size += 9;
        }
        if self.casa_stable != 0. {
            my_size += 9;
        }
        if self.casa_less_stable != 0. {
            my_size += 9;
        }
        if self.stable_b1 != 0. {
            my_size += 9;
        }
        if self.stable_b2 != 0. {
            my_size += 10;
        }
        if self.stable_b3 != 0. {
            my_size += 10;
        }
        if self.less_stable_b1 != 0. {
            my_size += 10;
        }
        if self.less_stable_b2 != 0. {
            my_size += 10;
        }
        if self.less_stable_b3 != 0. {
            my_size += 10;
        }
        if self.nwd_b1 != 0. {
            my_size += 10;
        }
        if self.nwd_b2 != 0. {
            my_size += 10;
        }
        if self.nwd_b3 != 0. {
            my_size += 10;
        }
        if !self.currency.is_empty() {
            my_size += ::protobuf::rt::string_size(24, &self.currency);
        }
        if self.nwd_stable_b1 != 0. {
            my_size += 10;
        }
        if self.nwd_stable_b2 != 0. {
            my_size += 10;
        }
        if self.nwd_stable_b3 != 0. {
            my_size += 10;
        }
        if self.nwd_less_stable_b1 != 0. {
            my_size += 10;
        }
        if self.nwd_less_stable_b2 != 0. {
            my_size += 10;
        }
        if self.nwd_less_stable_b3 != 0. {
            my_size += 10;
        }
        if self.ca_wt_int_rate != 0. {
            my_size += 10;
        }
        if self.sa_wt_int_rate != 0. {
            my_size += 10;
        }
        if self.td_wd_wt_int_rate != 0. {
            my_size += 10;
        }
        if self.td_nwd_wt_int_rate != 0. {
            my_size += 10;
        }
        if self.rd_wt_int_rate != 0. {
            my_size += 10;
        }
        if self.lien_amt != 0. {
            my_size += 10;
        }
        if self.bal_before_lien != 0. {
            my_size += 10;
        }
        if self.bal_after_lien != 0. {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.file_id.is_empty() {
            os.write_string(1, &self.file_id)?;
        }
        if self.ca != 0. {
            os.write_double(2, self.ca)?;
        }
        if self.sa != 0. {
            os.write_double(3, self.sa)?;
        }
        if self.td_wd != 0. {
            os.write_double(4, self.td_wd)?;
        }
        if self.td_nwd != 0. {
            os.write_double(5, self.td_nwd)?;
        }
        if self.rd != 0. {
            os.write_double(6, self.rd)?;
        }
        if self.tot_stable != 0. {
            os.write_double(7, self.tot_stable)?;
        }
        if self.tot_less_stable != 0. {
            os.write_double(8, self.tot_less_stable)?;
        }
        if self.ca_stable != 0. {
            os.write_double(9, self.ca_stable)?;
        }
        if self.ca_less_stable != 0. {
            os.write_double(10, self.ca_less_stable)?;
        }
        if self.sa_stable != 0. {
            os.write_double(11, self.sa_stable)?;
        }
        if self.sa_less_stable != 0. {
            os.write_double(12, self.sa_less_stable)?;
        }
        if self.casa_stable != 0. {
            os.write_double(13, self.casa_stable)?;
        }
        if self.casa_less_stable != 0. {
            os.write_double(14, self.casa_less_stable)?;
        }
        if self.stable_b1 != 0. {
            os.write_double(15, self.stable_b1)?;
        }
        if self.stable_b2 != 0. {
            os.write_double(16, self.stable_b2)?;
        }
        if self.stable_b3 != 0. {
            os.write_double(17, self.stable_b3)?;
        }
        if self.less_stable_b1 != 0. {
            os.write_double(18, self.less_stable_b1)?;
        }
        if self.less_stable_b2 != 0. {
            os.write_double(19, self.less_stable_b2)?;
        }
        if self.less_stable_b3 != 0. {
            os.write_double(20, self.less_stable_b3)?;
        }
        if self.nwd_b1 != 0. {
            os.write_double(21, self.nwd_b1)?;
        }
        if self.nwd_b2 != 0. {
            os.write_double(22, self.nwd_b2)?;
        }
        if self.nwd_b3 != 0. {
            os.write_double(23, self.nwd_b3)?;
        }
        if !self.currency.is_empty() {
            os.write_string(24, &self.currency)?;
        }
        if self.nwd_stable_b1 != 0. {
            os.write_double(25, self.nwd_stable_b1)?;
        }
        if self.nwd_stable_b2 != 0. {
            os.write_double(26, self.nwd_stable_b2)?;
        }
        if self.nwd_stable_b3 != 0. {
            os.write_double(27, self.nwd_stable_b3)?;
        }
        if self.nwd_less_stable_b1 != 0. {
            os.write_double(28, self.nwd_less_stable_b1)?;
        }
        if self.nwd_less_stable_b2 != 0. {
            os.write_double(29, self.nwd_less_stable_b2)?;
        }
        if self.nwd_less_stable_b3 != 0. {
            os.write_double(30, self.nwd_less_stable_b3)?;
        }
        if self.ca_wt_int_rate != 0. {
            os.write_double(31, self.ca_wt_int_rate)?;
        }
        if self.sa_wt_int_rate != 0. {
            os.write_double(32, self.sa_wt_int_rate)?;
        }
        if self.td_wd_wt_int_rate != 0. {
            os.write_double(33, self.td_wd_wt_int_rate)?;
        }
        if self.td_nwd_wt_int_rate != 0. {
            os.write_double(34, self.td_nwd_wt_int_rate)?;
        }
        if self.rd_wt_int_rate != 0. {
            os.write_double(35, self.rd_wt_int_rate)?;
        }
        if self.lien_amt != 0. {
            os.write_double(36, self.lien_amt)?;
        }
        if self.bal_before_lien != 0. {
            os.write_double(37, self.bal_before_lien)?;
        }
        if self.bal_after_lien != 0. {
            os.write_double(38, self.bal_after_lien)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OutputAccount {
        OutputAccount::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "file_id",
                |m: &OutputAccount| { &m.file_id },
                |m: &mut OutputAccount| { &mut m.file_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ca",
                |m: &OutputAccount| { &m.ca },
                |m: &mut OutputAccount| { &mut m.ca },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sa",
                |m: &OutputAccount| { &m.sa },
                |m: &mut OutputAccount| { &mut m.sa },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "td_wd",
                |m: &OutputAccount| { &m.td_wd },
                |m: &mut OutputAccount| { &mut m.td_wd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "td_nwd",
                |m: &OutputAccount| { &m.td_nwd },
                |m: &mut OutputAccount| { &mut m.td_nwd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "rd",
                |m: &OutputAccount| { &m.rd },
                |m: &mut OutputAccount| { &mut m.rd },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_stable",
                |m: &OutputAccount| { &m.tot_stable },
                |m: &mut OutputAccount| { &mut m.tot_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "tot_less_stable",
                |m: &OutputAccount| { &m.tot_less_stable },
                |m: &mut OutputAccount| { &mut m.tot_less_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ca_stable",
                |m: &OutputAccount| { &m.ca_stable },
                |m: &mut OutputAccount| { &mut m.ca_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ca_less_stable",
                |m: &OutputAccount| { &m.ca_less_stable },
                |m: &mut OutputAccount| { &mut m.ca_less_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sa_stable",
                |m: &OutputAccount| { &m.sa_stable },
                |m: &mut OutputAccount| { &mut m.sa_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sa_less_stable",
                |m: &OutputAccount| { &m.sa_less_stable },
                |m: &mut OutputAccount| { &mut m.sa_less_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "casa_stable",
                |m: &OutputAccount| { &m.casa_stable },
                |m: &mut OutputAccount| { &mut m.casa_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "casa_less_stable",
                |m: &OutputAccount| { &m.casa_less_stable },
                |m: &mut OutputAccount| { &mut m.casa_less_stable },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "stable_b1",
                |m: &OutputAccount| { &m.stable_b1 },
                |m: &mut OutputAccount| { &mut m.stable_b1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "stable_b2",
                |m: &OutputAccount| { &m.stable_b2 },
                |m: &mut OutputAccount| { &mut m.stable_b2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "stable_b3",
                |m: &OutputAccount| { &m.stable_b3 },
                |m: &mut OutputAccount| { &mut m.stable_b3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "less_stable_b1",
                |m: &OutputAccount| { &m.less_stable_b1 },
                |m: &mut OutputAccount| { &mut m.less_stable_b1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "less_stable_b2",
                |m: &OutputAccount| { &m.less_stable_b2 },
                |m: &mut OutputAccount| { &mut m.less_stable_b2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "less_stable_b3",
                |m: &OutputAccount| { &m.less_stable_b3 },
                |m: &mut OutputAccount| { &mut m.less_stable_b3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_b1",
                |m: &OutputAccount| { &m.nwd_b1 },
                |m: &mut OutputAccount| { &mut m.nwd_b1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_b2",
                |m: &OutputAccount| { &m.nwd_b2 },
                |m: &mut OutputAccount| { &mut m.nwd_b2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_b3",
                |m: &OutputAccount| { &m.nwd_b3 },
                |m: &mut OutputAccount| { &mut m.nwd_b3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency",
                |m: &OutputAccount| { &m.currency },
                |m: &mut OutputAccount| { &mut m.currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_stable_b1",
                |m: &OutputAccount| { &m.nwd_stable_b1 },
                |m: &mut OutputAccount| { &mut m.nwd_stable_b1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_stable_b2",
                |m: &OutputAccount| { &m.nwd_stable_b2 },
                |m: &mut OutputAccount| { &mut m.nwd_stable_b2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_stable_b3",
                |m: &OutputAccount| { &m.nwd_stable_b3 },
                |m: &mut OutputAccount| { &mut m.nwd_stable_b3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_less_stable_b1",
                |m: &OutputAccount| { &m.nwd_less_stable_b1 },
                |m: &mut OutputAccount| { &mut m.nwd_less_stable_b1 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_less_stable_b2",
                |m: &OutputAccount| { &m.nwd_less_stable_b2 },
                |m: &mut OutputAccount| { &mut m.nwd_less_stable_b2 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "nwd_less_stable_b3",
                |m: &OutputAccount| { &m.nwd_less_stable_b3 },
                |m: &mut OutputAccount| { &mut m.nwd_less_stable_b3 },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ca_wt_int_rate",
                |m: &OutputAccount| { &m.ca_wt_int_rate },
                |m: &mut OutputAccount| { &mut m.ca_wt_int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "sa_wt_int_rate",
                |m: &OutputAccount| { &m.sa_wt_int_rate },
                |m: &mut OutputAccount| { &mut m.sa_wt_int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "td_wd_wt_int_rate",
                |m: &OutputAccount| { &m.td_wd_wt_int_rate },
                |m: &mut OutputAccount| { &mut m.td_wd_wt_int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "td_nwd_wt_int_rate",
                |m: &OutputAccount| { &m.td_nwd_wt_int_rate },
                |m: &mut OutputAccount| { &mut m.td_nwd_wt_int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "rd_wt_int_rate",
                |m: &OutputAccount| { &m.rd_wt_int_rate },
                |m: &mut OutputAccount| { &mut m.rd_wt_int_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "lien_amt",
                |m: &OutputAccount| { &m.lien_amt },
                |m: &mut OutputAccount| { &mut m.lien_amt },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "bal_before_lien",
                |m: &OutputAccount| { &m.bal_before_lien },
                |m: &mut OutputAccount| { &mut m.bal_before_lien },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "bal_after_lien",
                |m: &OutputAccount| { &m.bal_after_lien },
                |m: &mut OutputAccount| { &mut m.bal_after_lien },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OutputAccount>(
                "OutputAccount",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OutputAccount {
        static instance: ::protobuf::rt::LazyV2<OutputAccount> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OutputAccount::new)
    }
}

impl ::protobuf::Clear for OutputAccount {
    fn clear(&mut self) {
        self.file_id.clear();
        self.ca = 0.;
        self.sa = 0.;
        self.td_wd = 0.;
        self.td_nwd = 0.;
        self.rd = 0.;
        self.tot_stable = 0.;
        self.tot_less_stable = 0.;
        self.ca_stable = 0.;
        self.ca_less_stable = 0.;
        self.sa_stable = 0.;
        self.sa_less_stable = 0.;
        self.casa_stable = 0.;
        self.casa_less_stable = 0.;
        self.stable_b1 = 0.;
        self.stable_b2 = 0.;
        self.stable_b3 = 0.;
        self.less_stable_b1 = 0.;
        self.less_stable_b2 = 0.;
        self.less_stable_b3 = 0.;
        self.nwd_b1 = 0.;
        self.nwd_b2 = 0.;
        self.nwd_b3 = 0.;
        self.currency.clear();
        self.nwd_stable_b1 = 0.;
        self.nwd_stable_b2 = 0.;
        self.nwd_stable_b3 = 0.;
        self.nwd_less_stable_b1 = 0.;
        self.nwd_less_stable_b2 = 0.;
        self.nwd_less_stable_b3 = 0.;
        self.ca_wt_int_rate = 0.;
        self.sa_wt_int_rate = 0.;
        self.td_wd_wt_int_rate = 0.;
        self.td_nwd_wt_int_rate = 0.;
        self.rd_wt_int_rate = 0.;
        self.lien_amt = 0.;
        self.bal_before_lien = 0.;
        self.bal_after_lien = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputAccount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputAccount {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccf-lcr.proto\"\xe7\t\n\rOutputAccount\x12\x17\n\x07file_id\x18\x01\
    \x20\x01(\tR\x06fileId\x12\x0e\n\x02ca\x18\x02\x20\x01(\x01R\x02ca\x12\
    \x0e\n\x02sa\x18\x03\x20\x01(\x01R\x02sa\x12\x13\n\x05td_wd\x18\x04\x20\
    \x01(\x01R\x04tdWd\x12\x15\n\x06td_nwd\x18\x05\x20\x01(\x01R\x05tdNwd\
    \x12\x0e\n\x02rd\x18\x06\x20\x01(\x01R\x02rd\x12\x1d\n\ntot_stable\x18\
    \x07\x20\x01(\x01R\ttotStable\x12&\n\x0ftot_less_stable\x18\x08\x20\x01(\
    \x01R\rtotLessStable\x12\x1b\n\tca_stable\x18\t\x20\x01(\x01R\x08caStabl\
    e\x12$\n\x0eca_less_stable\x18\n\x20\x01(\x01R\x0ccaLessStable\x12\x1b\n\
    \tsa_stable\x18\x0b\x20\x01(\x01R\x08saStable\x12$\n\x0esa_less_stable\
    \x18\x0c\x20\x01(\x01R\x0csaLessStable\x12\x1f\n\x0bcasa_stable\x18\r\
    \x20\x01(\x01R\ncasaStable\x12(\n\x10casa_less_stable\x18\x0e\x20\x01(\
    \x01R\x0ecasaLessStable\x12\x1b\n\tstable_b1\x18\x0f\x20\x01(\x01R\x08st\
    ableB1\x12\x1b\n\tstable_b2\x18\x10\x20\x01(\x01R\x08stableB2\x12\x1b\n\
    \tstable_b3\x18\x11\x20\x01(\x01R\x08stableB3\x12$\n\x0eless_stable_b1\
    \x18\x12\x20\x01(\x01R\x0clessStableB1\x12$\n\x0eless_stable_b2\x18\x13\
    \x20\x01(\x01R\x0clessStableB2\x12$\n\x0eless_stable_b3\x18\x14\x20\x01(\
    \x01R\x0clessStableB3\x12\x15\n\x06nwd_b1\x18\x15\x20\x01(\x01R\x05nwdB1\
    \x12\x15\n\x06nwd_b2\x18\x16\x20\x01(\x01R\x05nwdB2\x12\x15\n\x06nwd_b3\
    \x18\x17\x20\x01(\x01R\x05nwdB3\x12\x1a\n\x08currency\x18\x18\x20\x01(\t\
    R\x08currency\x12\"\n\rnwd_stable_b1\x18\x19\x20\x01(\x01R\x0bnwdStableB\
    1\x12\"\n\rnwd_stable_b2\x18\x1a\x20\x01(\x01R\x0bnwdStableB2\x12\"\n\rn\
    wd_stable_b3\x18\x1b\x20\x01(\x01R\x0bnwdStableB3\x12+\n\x12nwd_less_sta\
    ble_b1\x18\x1c\x20\x01(\x01R\x0fnwdLessStableB1\x12+\n\x12nwd_less_stabl\
    e_b2\x18\x1d\x20\x01(\x01R\x0fnwdLessStableB2\x12+\n\x12nwd_less_stable_\
    b3\x18\x1e\x20\x01(\x01R\x0fnwdLessStableB3\x12#\n\x0eca_wt_int_rate\x18\
    \x1f\x20\x01(\x01R\x0bcaWtIntRate\x12#\n\x0esa_wt_int_rate\x18\x20\x20\
    \x01(\x01R\x0bsaWtIntRate\x12(\n\x11td_wd_wt_int_rate\x18!\x20\x01(\x01R\
    \rtdWdWtIntRate\x12*\n\x12td_nwd_wt_int_rate\x18\"\x20\x01(\x01R\x0etdNw\
    dWtIntRate\x12#\n\x0erd_wt_int_rate\x18#\x20\x01(\x01R\x0brdWtIntRate\
    \x12\x19\n\x08lien_amt\x18$\x20\x01(\x01R\x07lienAmt\x12&\n\x0fbal_befor\
    e_lien\x18%\x20\x01(\x01R\rbalBeforeLien\x12$\n\x0ebal_after_lien\x18&\
    \x20\x01(\x01R\x0cbalAfterLienb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
