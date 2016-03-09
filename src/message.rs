// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct DHTMsg {
    // message fields
    field_type: ::std::option::Option<DHTMsg_Type>,
    join_msg: ::protobuf::SingularPtrField<JoinMsg>,
    dump_msg: ::protobuf::SingularPtrField<DumpMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DHTMsg {
    pub fn new() -> DHTMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DHTMsg {
        static mut instance: ::protobuf::lazy::Lazy<DHTMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DHTMsg,
        };
        unsafe {
            instance.get(|| {
                DHTMsg {
                    field_type: ::std::option::Option::None,
                    join_msg: ::protobuf::SingularPtrField::none(),
                    dump_msg: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .DHTMsg.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DHTMsg_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> DHTMsg_Type {
        self.field_type.unwrap_or(DHTMsg_Type::HEARTBEAT)
    }

    // optional .JoinMsg join_msg = 2;

    pub fn clear_join_msg(&mut self) {
        self.join_msg.clear();
    }

    pub fn has_join_msg(&self) -> bool {
        self.join_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_join_msg(&mut self, v: JoinMsg) {
        self.join_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_join_msg<'a>(&'a mut self) -> &'a mut JoinMsg {
        if self.join_msg.is_none() {
            self.join_msg.set_default();
        };
        self.join_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_join_msg(&mut self) -> JoinMsg {
        self.join_msg.take().unwrap_or_else(|| JoinMsg::new())
    }

    pub fn get_join_msg<'a>(&'a self) -> &'a JoinMsg {
        self.join_msg.as_ref().unwrap_or_else(|| JoinMsg::default_instance())
    }

    // optional .DumpMsg dump_msg = 3;

    pub fn clear_dump_msg(&mut self) {
        self.dump_msg.clear();
    }

    pub fn has_dump_msg(&self) -> bool {
        self.dump_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dump_msg(&mut self, v: DumpMsg) {
        self.dump_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dump_msg<'a>(&'a mut self) -> &'a mut DumpMsg {
        if self.dump_msg.is_none() {
            self.dump_msg.set_default();
        };
        self.dump_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_dump_msg(&mut self) -> DumpMsg {
        self.dump_msg.take().unwrap_or_else(|| DumpMsg::new())
    }

    pub fn get_dump_msg<'a>(&'a self) -> &'a DumpMsg {
        self.dump_msg.as_ref().unwrap_or_else(|| DumpMsg::default_instance())
    }
}

impl ::protobuf::Message for DHTMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.join_msg));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dump_msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.join_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.dump_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.join_msg.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.dump_msg.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DHTMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DHTMsg {
    fn new() -> DHTMsg {
        DHTMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<DHTMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    DHTMsg::has_field_type,
                    DHTMsg::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "join_msg",
                    DHTMsg::has_join_msg,
                    DHTMsg::get_join_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "dump_msg",
                    DHTMsg::has_dump_msg,
                    DHTMsg::get_dump_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DHTMsg>(
                    "DHTMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DHTMsg {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_join_msg();
        self.clear_dump_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DHTMsg {
    fn eq(&self, other: &DHTMsg) -> bool {
        self.field_type == other.field_type &&
        self.join_msg == other.join_msg &&
        self.dump_msg == other.dump_msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DHTMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DHTMsg_Type {
    HEARTBEAT = 0,
    JOIN = 1,
    DUMP = 2,
}

impl ::protobuf::ProtobufEnum for DHTMsg_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DHTMsg_Type> {
        match value {
            0 => ::std::option::Option::Some(DHTMsg_Type::HEARTBEAT),
            1 => ::std::option::Option::Some(DHTMsg_Type::JOIN),
            2 => ::std::option::Option::Some(DHTMsg_Type::DUMP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DHTMsg_Type] = &[
            DHTMsg_Type::HEARTBEAT,
            DHTMsg_Type::JOIN,
            DHTMsg_Type::DUMP,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DHTMsg_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DHTMsg_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DHTMsg_Type {
}

#[derive(Clone,Default)]
pub struct JoinMsg {
    // message fields
    dht_address: ::protobuf::SingularField<::std::string::String>,
    application_address: ::protobuf::SingularField<::std::string::String>,
    tokens: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl JoinMsg {
    pub fn new() -> JoinMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JoinMsg {
        static mut instance: ::protobuf::lazy::Lazy<JoinMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JoinMsg,
        };
        unsafe {
            instance.get(|| {
                JoinMsg {
                    dht_address: ::protobuf::SingularField::none(),
                    application_address: ::protobuf::SingularField::none(),
                    tokens: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string dht_address = 1;

    pub fn clear_dht_address(&mut self) {
        self.dht_address.clear();
    }

    pub fn has_dht_address(&self) -> bool {
        self.dht_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dht_address(&mut self, v: ::std::string::String) {
        self.dht_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dht_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.dht_address.is_none() {
            self.dht_address.set_default();
        };
        self.dht_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_dht_address(&mut self) -> ::std::string::String {
        self.dht_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dht_address<'a>(&'a self) -> &'a str {
        match self.dht_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string application_address = 2;

    pub fn clear_application_address(&mut self) {
        self.application_address.clear();
    }

    pub fn has_application_address(&self) -> bool {
        self.application_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_address(&mut self, v: ::std::string::String) {
        self.application_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_application_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.application_address.is_none() {
            self.application_address.set_default();
        };
        self.application_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_application_address(&mut self) -> ::std::string::String {
        self.application_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_application_address<'a>(&'a self) -> &'a str {
        match self.application_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated int64 tokens = 3;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::std::vec::Vec<i64>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.tokens, ::std::vec::Vec::new())
    }

    pub fn get_tokens<'a>(&'a self) -> &'a [i64] {
        &self.tokens
    }
}

impl ::protobuf::Message for JoinMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dht_address));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.application_address));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.tokens));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.dht_address.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.application_address.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.tokens.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dht_address.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.application_address.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in self.tokens.iter() {
            try!(os.write_int64(3, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<JoinMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for JoinMsg {
    fn new() -> JoinMsg {
        JoinMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<JoinMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "dht_address",
                    JoinMsg::has_dht_address,
                    JoinMsg::get_dht_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "application_address",
                    JoinMsg::has_application_address,
                    JoinMsg::get_application_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "tokens",
                    JoinMsg::get_tokens,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JoinMsg>(
                    "JoinMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JoinMsg {
    fn clear(&mut self) {
        self.clear_dht_address();
        self.clear_application_address();
        self.clear_tokens();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for JoinMsg {
    fn eq(&self, other: &JoinMsg) -> bool {
        self.dht_address == other.dht_address &&
        self.application_address == other.application_address &&
        self.tokens == other.tokens &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for JoinMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DumpMsg {
    // message fields
    dht_address: ::protobuf::SingularField<::std::string::String>,
    tokens: ::std::vec::Vec<i64>,
    lookup_table: ::protobuf::RepeatedField<DumpMsg_LookupTableEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DumpMsg {
    pub fn new() -> DumpMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DumpMsg {
        static mut instance: ::protobuf::lazy::Lazy<DumpMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DumpMsg,
        };
        unsafe {
            instance.get(|| {
                DumpMsg {
                    dht_address: ::protobuf::SingularField::none(),
                    tokens: ::std::vec::Vec::new(),
                    lookup_table: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string dht_address = 1;

    pub fn clear_dht_address(&mut self) {
        self.dht_address.clear();
    }

    pub fn has_dht_address(&self) -> bool {
        self.dht_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dht_address(&mut self, v: ::std::string::String) {
        self.dht_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dht_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.dht_address.is_none() {
            self.dht_address.set_default();
        };
        self.dht_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_dht_address(&mut self) -> ::std::string::String {
        self.dht_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dht_address<'a>(&'a self) -> &'a str {
        match self.dht_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated int64 tokens = 2;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::std::vec::Vec<i64>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.tokens, ::std::vec::Vec::new())
    }

    pub fn get_tokens<'a>(&'a self) -> &'a [i64] {
        &self.tokens
    }

    // repeated .DumpMsg.LookupTableEntry lookup_table = 3;

    pub fn clear_lookup_table(&mut self) {
        self.lookup_table.clear();
    }

    // Param is passed by value, moved
    pub fn set_lookup_table(&mut self, v: ::protobuf::RepeatedField<DumpMsg_LookupTableEntry>) {
        self.lookup_table = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lookup_table<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DumpMsg_LookupTableEntry> {
        &mut self.lookup_table
    }

    // Take field
    pub fn take_lookup_table(&mut self) -> ::protobuf::RepeatedField<DumpMsg_LookupTableEntry> {
        ::std::mem::replace(&mut self.lookup_table, ::protobuf::RepeatedField::new())
    }

    pub fn get_lookup_table<'a>(&'a self) -> &'a [DumpMsg_LookupTableEntry] {
        &self.lookup_table
    }
}

impl ::protobuf::Message for DumpMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dht_address));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.tokens));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lookup_table));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.dht_address.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.tokens.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lookup_table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dht_address.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.tokens.iter() {
            try!(os.write_int64(2, *v));
        };
        for v in self.lookup_table.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DumpMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DumpMsg {
    fn new() -> DumpMsg {
        DumpMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<DumpMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "dht_address",
                    DumpMsg::has_dht_address,
                    DumpMsg::get_dht_address,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "tokens",
                    DumpMsg::get_tokens,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "lookup_table",
                    DumpMsg::get_lookup_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DumpMsg>(
                    "DumpMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DumpMsg {
    fn clear(&mut self) {
        self.clear_dht_address();
        self.clear_tokens();
        self.clear_lookup_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DumpMsg {
    fn eq(&self, other: &DumpMsg) -> bool {
        self.dht_address == other.dht_address &&
        self.tokens == other.tokens &&
        self.lookup_table == other.lookup_table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DumpMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DumpMsg_LookupTableEntry {
    // message fields
    key: ::std::option::Option<i64>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DumpMsg_LookupTableEntry {
    pub fn new() -> DumpMsg_LookupTableEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DumpMsg_LookupTableEntry {
        static mut instance: ::protobuf::lazy::Lazy<DumpMsg_LookupTableEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DumpMsg_LookupTableEntry,
        };
        unsafe {
            instance.get(|| {
                DumpMsg_LookupTableEntry {
                    key: ::std::option::Option::None,
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 key = 1;

    pub fn clear_key(&mut self) {
        self.key = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: i64) {
        self.key = ::std::option::Option::Some(v);
    }

    pub fn get_key<'a>(&self) -> i64 {
        self.key.unwrap_or(0)
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for DumpMsg_LookupTableEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.key = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DumpMsg_LookupTableEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DumpMsg_LookupTableEntry {
    fn new() -> DumpMsg_LookupTableEntry {
        DumpMsg_LookupTableEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<DumpMsg_LookupTableEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "key",
                    DumpMsg_LookupTableEntry::has_key,
                    DumpMsg_LookupTableEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    DumpMsg_LookupTableEntry::has_value,
                    DumpMsg_LookupTableEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DumpMsg_LookupTableEntry>(
                    "DumpMsg_LookupTableEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DumpMsg_LookupTableEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DumpMsg_LookupTableEntry {
    fn eq(&self, other: &DumpMsg_LookupTableEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DumpMsg_LookupTableEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CoterieMsg {
    // message fields
    field_type: ::std::option::Option<CoterieMsg_Type>,
    write_entities_msg: ::protobuf::SingularPtrField<WriteEntitiesMsg>,
    write_entity_msg: ::protobuf::SingularPtrField<WriteEntityMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CoterieMsg {
    pub fn new() -> CoterieMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CoterieMsg {
        static mut instance: ::protobuf::lazy::Lazy<CoterieMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CoterieMsg,
        };
        unsafe {
            instance.get(|| {
                CoterieMsg {
                    field_type: ::std::option::Option::None,
                    write_entities_msg: ::protobuf::SingularPtrField::none(),
                    write_entity_msg: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CoterieMsg.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: CoterieMsg_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> CoterieMsg_Type {
        self.field_type.unwrap_or(CoterieMsg_Type::CLOSE_WRITE_STREAM)
    }

    // optional .WriteEntitiesMsg write_entities_msg = 2;

    pub fn clear_write_entities_msg(&mut self) {
        self.write_entities_msg.clear();
    }

    pub fn has_write_entities_msg(&self) -> bool {
        self.write_entities_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_write_entities_msg(&mut self, v: WriteEntitiesMsg) {
        self.write_entities_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_write_entities_msg<'a>(&'a mut self) -> &'a mut WriteEntitiesMsg {
        if self.write_entities_msg.is_none() {
            self.write_entities_msg.set_default();
        };
        self.write_entities_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_write_entities_msg(&mut self) -> WriteEntitiesMsg {
        self.write_entities_msg.take().unwrap_or_else(|| WriteEntitiesMsg::new())
    }

    pub fn get_write_entities_msg<'a>(&'a self) -> &'a WriteEntitiesMsg {
        self.write_entities_msg.as_ref().unwrap_or_else(|| WriteEntitiesMsg::default_instance())
    }

    // optional .WriteEntityMsg write_entity_msg = 3;

    pub fn clear_write_entity_msg(&mut self) {
        self.write_entity_msg.clear();
    }

    pub fn has_write_entity_msg(&self) -> bool {
        self.write_entity_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_write_entity_msg(&mut self, v: WriteEntityMsg) {
        self.write_entity_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_write_entity_msg<'a>(&'a mut self) -> &'a mut WriteEntityMsg {
        if self.write_entity_msg.is_none() {
            self.write_entity_msg.set_default();
        };
        self.write_entity_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_write_entity_msg(&mut self) -> WriteEntityMsg {
        self.write_entity_msg.take().unwrap_or_else(|| WriteEntityMsg::new())
    }

    pub fn get_write_entity_msg<'a>(&'a self) -> &'a WriteEntityMsg {
        self.write_entity_msg.as_ref().unwrap_or_else(|| WriteEntityMsg::default_instance())
    }
}

impl ::protobuf::Message for CoterieMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.write_entities_msg));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.write_entity_msg));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.write_entities_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.write_entity_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.write_entities_msg.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.write_entity_msg.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CoterieMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CoterieMsg {
    fn new() -> CoterieMsg {
        CoterieMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CoterieMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    CoterieMsg::has_field_type,
                    CoterieMsg::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "write_entities_msg",
                    CoterieMsg::has_write_entities_msg,
                    CoterieMsg::get_write_entities_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "write_entity_msg",
                    CoterieMsg::has_write_entity_msg,
                    CoterieMsg::get_write_entity_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CoterieMsg>(
                    "CoterieMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CoterieMsg {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_write_entities_msg();
        self.clear_write_entity_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CoterieMsg {
    fn eq(&self, other: &CoterieMsg) -> bool {
        self.field_type == other.field_type &&
        self.write_entities_msg == other.write_entities_msg &&
        self.write_entity_msg == other.write_entity_msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CoterieMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CoterieMsg_Type {
    CLOSE_WRITE_STREAM = 0,
    OPEN_WRITE_STREAM = 1,
    WRITE_ENTITIES = 2,
    WRITE_ENTITY = 3,
}

impl ::protobuf::ProtobufEnum for CoterieMsg_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CoterieMsg_Type> {
        match value {
            0 => ::std::option::Option::Some(CoterieMsg_Type::CLOSE_WRITE_STREAM),
            1 => ::std::option::Option::Some(CoterieMsg_Type::OPEN_WRITE_STREAM),
            2 => ::std::option::Option::Some(CoterieMsg_Type::WRITE_ENTITIES),
            3 => ::std::option::Option::Some(CoterieMsg_Type::WRITE_ENTITY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CoterieMsg_Type] = &[
            CoterieMsg_Type::CLOSE_WRITE_STREAM,
            CoterieMsg_Type::OPEN_WRITE_STREAM,
            CoterieMsg_Type::WRITE_ENTITIES,
            CoterieMsg_Type::WRITE_ENTITY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CoterieMsg_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CoterieMsg_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CoterieMsg_Type {
}

#[derive(Clone,Default)]
pub struct WriteEntitiesMsg {
    // message fields
    entity: ::protobuf::RepeatedField<Entity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteEntitiesMsg {
    pub fn new() -> WriteEntitiesMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteEntitiesMsg {
        static mut instance: ::protobuf::lazy::Lazy<WriteEntitiesMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteEntitiesMsg,
        };
        unsafe {
            instance.get(|| {
                WriteEntitiesMsg {
                    entity: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Entity entity = 1;

    pub fn clear_entity(&mut self) {
        self.entity.clear();
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: ::protobuf::RepeatedField<Entity>) {
        self.entity = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entity<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Entity> {
        &mut self.entity
    }

    // Take field
    pub fn take_entity(&mut self) -> ::protobuf::RepeatedField<Entity> {
        ::std::mem::replace(&mut self.entity, ::protobuf::RepeatedField::new())
    }

    pub fn get_entity<'a>(&'a self) -> &'a [Entity] {
        &self.entity
    }
}

impl ::protobuf::Message for WriteEntitiesMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entity));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.entity.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.entity.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WriteEntitiesMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteEntitiesMsg {
    fn new() -> WriteEntitiesMsg {
        WriteEntitiesMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteEntitiesMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entity",
                    WriteEntitiesMsg::get_entity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteEntitiesMsg>(
                    "WriteEntitiesMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteEntitiesMsg {
    fn clear(&mut self) {
        self.clear_entity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteEntitiesMsg {
    fn eq(&self, other: &WriteEntitiesMsg) -> bool {
        self.entity == other.entity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteEntitiesMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WriteEntityMsg {
    // message fields
    entity: ::protobuf::SingularPtrField<Entity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl WriteEntityMsg {
    pub fn new() -> WriteEntityMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteEntityMsg {
        static mut instance: ::protobuf::lazy::Lazy<WriteEntityMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteEntityMsg,
        };
        unsafe {
            instance.get(|| {
                WriteEntityMsg {
                    entity: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Entity entity = 1;

    pub fn clear_entity(&mut self) {
        self.entity.clear();
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: Entity) {
        self.entity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity<'a>(&'a mut self) -> &'a mut Entity {
        if self.entity.is_none() {
            self.entity.set_default();
        };
        self.entity.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity(&mut self) -> Entity {
        self.entity.take().unwrap_or_else(|| Entity::new())
    }

    pub fn get_entity<'a>(&'a self) -> &'a Entity {
        self.entity.as_ref().unwrap_or_else(|| Entity::default_instance())
    }
}

impl ::protobuf::Message for WriteEntityMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.entity));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.entity.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WriteEntityMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteEntityMsg {
    fn new() -> WriteEntityMsg {
        WriteEntityMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteEntityMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "entity",
                    WriteEntityMsg::has_entity,
                    WriteEntityMsg::get_entity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteEntityMsg>(
                    "WriteEntityMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteEntityMsg {
    fn clear(&mut self) {
        self.clear_entity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WriteEntityMsg {
    fn eq(&self, other: &WriteEntityMsg) -> bool {
        self.entity == other.entity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WriteEntityMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Entity {
    // message fields
    entity: ::protobuf::RepeatedField<Entity_EntityEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Entity {
    pub fn new() -> Entity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Entity {
        static mut instance: ::protobuf::lazy::Lazy<Entity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Entity,
        };
        unsafe {
            instance.get(|| {
                Entity {
                    entity: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Entity.EntityEntry entity = 1;

    pub fn clear_entity(&mut self) {
        self.entity.clear();
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: ::protobuf::RepeatedField<Entity_EntityEntry>) {
        self.entity = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entity<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Entity_EntityEntry> {
        &mut self.entity
    }

    // Take field
    pub fn take_entity(&mut self) -> ::protobuf::RepeatedField<Entity_EntityEntry> {
        ::std::mem::replace(&mut self.entity, ::protobuf::RepeatedField::new())
    }

    pub fn get_entity<'a>(&'a self) -> &'a [Entity_EntityEntry] {
        &self.entity
    }
}

impl ::protobuf::Message for Entity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entity));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.entity.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.entity.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Entity>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Entity {
    fn new() -> Entity {
        Entity::new()
    }

    fn descriptor_static(_: ::std::option::Option<Entity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entity",
                    Entity::get_entity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Entity>(
                    "Entity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Entity {
    fn clear(&mut self) {
        self.clear_entity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.entity == other.entity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Entity_EntityEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Entity_EntityEntry {
    pub fn new() -> Entity_EntityEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Entity_EntityEntry {
        static mut instance: ::protobuf::lazy::Lazy<Entity_EntityEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Entity_EntityEntry,
        };
        unsafe {
            instance.get(|| {
                Entity_EntityEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Entity_EntityEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Entity_EntityEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Entity_EntityEntry {
    fn new() -> Entity_EntityEntry {
        Entity_EntityEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Entity_EntityEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Entity_EntityEntry::has_key,
                    Entity_EntityEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    Entity_EntityEntry::has_value,
                    Entity_EntityEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Entity_EntityEntry>(
                    "Entity_EntityEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Entity_EntityEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Entity_EntityEntry {
    fn eq(&self, other: &Entity_EntityEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Entity_EntityEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x9f, 0x01, 0x0a, 0x06, 0x44, 0x48, 0x54,
    0x4d, 0x73, 0x67, 0x12, 0x20, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x0c, 0x2e, 0x44, 0x48, 0x54, 0x4d, 0x73, 0x67, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x08, 0x6a, 0x6f, 0x69, 0x6e, 0x5f, 0x6d, 0x73,
    0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x4d, 0x73,
    0x67, 0x52, 0x07, 0x6a, 0x6f, 0x69, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x23, 0x0a, 0x08, 0x64, 0x75,
    0x6d, 0x70, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x44,
    0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x52, 0x07, 0x64, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x22,
    0x29, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x48, 0x45, 0x41, 0x52, 0x54,
    0x42, 0x45, 0x41, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4a, 0x4f, 0x49, 0x4e, 0x10, 0x01,
    0x12, 0x08, 0x0a, 0x04, 0x44, 0x55, 0x4d, 0x50, 0x10, 0x02, 0x22, 0x73, 0x0a, 0x07, 0x4a, 0x6f,
    0x69, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x68, 0x74, 0x5f, 0x61, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x64, 0x68, 0x74, 0x41,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x2f, 0x0a, 0x13, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x12, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x03, 0x52, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x22,
    0xc0, 0x01, 0x0a, 0x07, 0x44, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x12, 0x1f, 0x0a, 0x0b, 0x64,
    0x68, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0a, 0x64, 0x68, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x16, 0x0a, 0x06,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x03, 0x52, 0x06, 0x74, 0x6f,
    0x6b, 0x65, 0x6e, 0x73, 0x12, 0x3c, 0x0a, 0x0c, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x5f, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x44, 0x75, 0x6d,
    0x70, 0x4d, 0x73, 0x67, 0x2e, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0b, 0x6c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x1a, 0x3e, 0x0a, 0x10, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x03, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02,
    0x38, 0x01, 0x22, 0x8b, 0x02, 0x0a, 0x0a, 0x43, 0x6f, 0x74, 0x65, 0x72, 0x69, 0x65, 0x4d, 0x73,
    0x67, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x10, 0x2e, 0x43, 0x6f, 0x74, 0x65, 0x72, 0x69, 0x65, 0x4d, 0x73, 0x67, 0x2e, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x3f, 0x0a, 0x12, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74,
    0x69, 0x65, 0x73, 0x4d, 0x73, 0x67, 0x52, 0x10, 0x77, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e, 0x74,
    0x69, 0x74, 0x69, 0x65, 0x73, 0x4d, 0x73, 0x67, 0x12, 0x39, 0x0a, 0x10, 0x77, 0x72, 0x69, 0x74,
    0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x6d, 0x73, 0x67, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4d, 0x73, 0x67, 0x52, 0x0e, 0x77, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4d, 0x73, 0x67, 0x22, 0x5b, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x12, 0x43,
    0x4c, 0x4f, 0x53, 0x45, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41,
    0x4d, 0x10, 0x00, 0x12, 0x15, 0x0a, 0x11, 0x4f, 0x50, 0x45, 0x4e, 0x5f, 0x57, 0x52, 0x49, 0x54,
    0x45, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x57, 0x52,
    0x49, 0x54, 0x45, 0x5f, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x49, 0x45, 0x53, 0x10, 0x02, 0x12, 0x10,
    0x0a, 0x0c, 0x57, 0x52, 0x49, 0x54, 0x45, 0x5f, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x59, 0x10, 0x03,
    0x22, 0x33, 0x0a, 0x10, 0x57, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65,
    0x73, 0x4d, 0x73, 0x67, 0x12, 0x1f, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x52, 0x06, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x31, 0x0a, 0x0e, 0x57, 0x72, 0x69, 0x74, 0x65, 0x45, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x4d, 0x73, 0x67, 0x12, 0x1f, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x07, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x70, 0x0a, 0x06, 0x45, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x12, 0x2b, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x45, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x1a,
    0x39, 0x0a, 0x0b, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10,
    0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79,
    0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x42, 0x02, 0x48, 0x01, 0x4a, 0xa4,
    0x0d, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x01, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x01, 0x16, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x0d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x04, 0x08, 0x08, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x06, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x06, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x06, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x07, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x07, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x07, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0a, 0x08,
    0x08, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0a, 0x08, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x0d, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x0b, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0b, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0c, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0f, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x08, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x10, 0x08, 0x0f, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x10, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x11, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x11, 0x08,
    0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x0f, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x12, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x12, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x12, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x20,
    0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x15, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x16, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x16, 0x08, 0x15, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x16, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0f,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x17, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x17, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x08,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x18, 0x08, 0x17, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x18, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x1a, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x1b, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1b,
    0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x04, 0x00, 0x12, 0x04, 0x1c, 0x08, 0x21, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x0d, 0x11, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x10, 0x27, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x22, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1d, 0x25, 0x26, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1e, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x03, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x20, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x20, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x23, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x23, 0x08, 0x21, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x23, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x14, 0x15,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x24, 0x08, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x24, 0x08, 0x23, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x19, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x24, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03,
    0x25, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x25, 0x08,
    0x24, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x25, 0x08, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x25, 0x17, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x25, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x29, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x08, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2d, 0x08, 0x2c, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x0f, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2d, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x30,
    0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x31, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x31, 0x08, 0x30, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x31, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x31, 0x24, 0x25, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
