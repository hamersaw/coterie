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
    heartbeat_msg: ::protobuf::SingularPtrField<HeartbeatMsg>,
    join_msg: ::protobuf::SingularPtrField<JoinMsg>,
    peer_table_dump_msg: ::protobuf::SingularPtrField<PeerTableDumpMsg>,
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
                    heartbeat_msg: ::protobuf::SingularPtrField::none(),
                    join_msg: ::protobuf::SingularPtrField::none(),
                    peer_table_dump_msg: ::protobuf::SingularPtrField::none(),
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

    // optional .HeartbeatMsg heartbeat_msg = 2;

    pub fn clear_heartbeat_msg(&mut self) {
        self.heartbeat_msg.clear();
    }

    pub fn has_heartbeat_msg(&self) -> bool {
        self.heartbeat_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_heartbeat_msg(&mut self, v: HeartbeatMsg) {
        self.heartbeat_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_heartbeat_msg<'a>(&'a mut self) -> &'a mut HeartbeatMsg {
        if self.heartbeat_msg.is_none() {
            self.heartbeat_msg.set_default();
        };
        self.heartbeat_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_heartbeat_msg(&mut self) -> HeartbeatMsg {
        self.heartbeat_msg.take().unwrap_or_else(|| HeartbeatMsg::new())
    }

    pub fn get_heartbeat_msg<'a>(&'a self) -> &'a HeartbeatMsg {
        self.heartbeat_msg.as_ref().unwrap_or_else(|| HeartbeatMsg::default_instance())
    }

    // optional .JoinMsg join_msg = 3;

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

    // optional .PeerTableDumpMsg peer_table_dump_msg = 4;

    pub fn clear_peer_table_dump_msg(&mut self) {
        self.peer_table_dump_msg.clear();
    }

    pub fn has_peer_table_dump_msg(&self) -> bool {
        self.peer_table_dump_msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_table_dump_msg(&mut self, v: PeerTableDumpMsg) {
        self.peer_table_dump_msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_table_dump_msg<'a>(&'a mut self) -> &'a mut PeerTableDumpMsg {
        if self.peer_table_dump_msg.is_none() {
            self.peer_table_dump_msg.set_default();
        };
        self.peer_table_dump_msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_table_dump_msg(&mut self) -> PeerTableDumpMsg {
        self.peer_table_dump_msg.take().unwrap_or_else(|| PeerTableDumpMsg::new())
    }

    pub fn get_peer_table_dump_msg<'a>(&'a self) -> &'a PeerTableDumpMsg {
        self.peer_table_dump_msg.as_ref().unwrap_or_else(|| PeerTableDumpMsg::default_instance())
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.heartbeat_msg));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.join_msg));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_table_dump_msg));
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
        for value in self.heartbeat_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.join_msg.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.peer_table_dump_msg.iter() {
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
        if let Some(v) = self.heartbeat_msg.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.join_msg.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.peer_table_dump_msg.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                    "heartbeat_msg",
                    DHTMsg::has_heartbeat_msg,
                    DHTMsg::get_heartbeat_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "join_msg",
                    DHTMsg::has_join_msg,
                    DHTMsg::get_join_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer_table_dump_msg",
                    DHTMsg::has_peer_table_dump_msg,
                    DHTMsg::get_peer_table_dump_msg,
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
        self.clear_heartbeat_msg();
        self.clear_join_msg();
        self.clear_peer_table_dump_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DHTMsg {
    fn eq(&self, other: &DHTMsg) -> bool {
        self.field_type == other.field_type &&
        self.heartbeat_msg == other.heartbeat_msg &&
        self.join_msg == other.join_msg &&
        self.peer_table_dump_msg == other.peer_table_dump_msg &&
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
    PEER_TABLE_DUMP = 2,
}

impl ::protobuf::ProtobufEnum for DHTMsg_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DHTMsg_Type> {
        match value {
            0 => ::std::option::Option::Some(DHTMsg_Type::HEARTBEAT),
            1 => ::std::option::Option::Some(DHTMsg_Type::JOIN),
            2 => ::std::option::Option::Some(DHTMsg_Type::PEER_TABLE_DUMP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DHTMsg_Type] = &[
            DHTMsg_Type::HEARTBEAT,
            DHTMsg_Type::JOIN,
            DHTMsg_Type::PEER_TABLE_DUMP,
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
pub struct HeartbeatMsg {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl HeartbeatMsg {
    pub fn new() -> HeartbeatMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeartbeatMsg {
        static mut instance: ::protobuf::lazy::Lazy<HeartbeatMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartbeatMsg,
        };
        unsafe {
            instance.get(|| {
                HeartbeatMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for HeartbeatMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<HeartbeatMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HeartbeatMsg {
    fn new() -> HeartbeatMsg {
        HeartbeatMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeartbeatMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<HeartbeatMsg>(
                    "HeartbeatMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeartbeatMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HeartbeatMsg {
    fn eq(&self, other: &HeartbeatMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HeartbeatMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct JoinMsg {
    // message fields
    tokens: ::std::vec::Vec<i64>,
    address: ::protobuf::SingularField<::std::string::String>,
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
                    tokens: ::std::vec::Vec::new(),
                    address: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int64 tokens = 1;

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

    // optional string address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
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
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.tokens));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address));
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
        for value in self.tokens.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.address.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.tokens.iter() {
            try!(os.write_int64(1, *v));
        };
        if let Some(v) = self.address.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "tokens",
                    JoinMsg::get_tokens,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    JoinMsg::has_address,
                    JoinMsg::get_address,
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
        self.clear_tokens();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for JoinMsg {
    fn eq(&self, other: &JoinMsg) -> bool {
        self.tokens == other.tokens &&
        self.address == other.address &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for JoinMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerTableDumpMsg {
    // message fields
    peer_table: ::protobuf::RepeatedField<PeerTableDumpMsg_PeerTableEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PeerTableDumpMsg {
    pub fn new() -> PeerTableDumpMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerTableDumpMsg {
        static mut instance: ::protobuf::lazy::Lazy<PeerTableDumpMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerTableDumpMsg,
        };
        unsafe {
            instance.get(|| {
                PeerTableDumpMsg {
                    peer_table: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .PeerTableDumpMsg.PeerTableEntry peer_table = 1;

    pub fn clear_peer_table(&mut self) {
        self.peer_table.clear();
    }

    // Param is passed by value, moved
    pub fn set_peer_table(&mut self, v: ::protobuf::RepeatedField<PeerTableDumpMsg_PeerTableEntry>) {
        self.peer_table = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peer_table<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<PeerTableDumpMsg_PeerTableEntry> {
        &mut self.peer_table
    }

    // Take field
    pub fn take_peer_table(&mut self) -> ::protobuf::RepeatedField<PeerTableDumpMsg_PeerTableEntry> {
        ::std::mem::replace(&mut self.peer_table, ::protobuf::RepeatedField::new())
    }

    pub fn get_peer_table<'a>(&'a self) -> &'a [PeerTableDumpMsg_PeerTableEntry] {
        &self.peer_table
    }
}

impl ::protobuf::Message for PeerTableDumpMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.peer_table));
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
        for value in self.peer_table.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.peer_table.iter() {
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
        ::std::any::TypeId::of::<PeerTableDumpMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PeerTableDumpMsg {
    fn new() -> PeerTableDumpMsg {
        PeerTableDumpMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerTableDumpMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "peer_table",
                    PeerTableDumpMsg::get_peer_table,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerTableDumpMsg>(
                    "PeerTableDumpMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerTableDumpMsg {
    fn clear(&mut self) {
        self.clear_peer_table();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PeerTableDumpMsg {
    fn eq(&self, other: &PeerTableDumpMsg) -> bool {
        self.peer_table == other.peer_table &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerTableDumpMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerTableDumpMsg_PeerTableEntry {
    // message fields
    key: ::std::option::Option<i64>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PeerTableDumpMsg_PeerTableEntry {
    pub fn new() -> PeerTableDumpMsg_PeerTableEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerTableDumpMsg_PeerTableEntry {
        static mut instance: ::protobuf::lazy::Lazy<PeerTableDumpMsg_PeerTableEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerTableDumpMsg_PeerTableEntry,
        };
        unsafe {
            instance.get(|| {
                PeerTableDumpMsg_PeerTableEntry {
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

impl ::protobuf::Message for PeerTableDumpMsg_PeerTableEntry {
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
        ::std::any::TypeId::of::<PeerTableDumpMsg_PeerTableEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PeerTableDumpMsg_PeerTableEntry {
    fn new() -> PeerTableDumpMsg_PeerTableEntry {
        PeerTableDumpMsg_PeerTableEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerTableDumpMsg_PeerTableEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "key",
                    PeerTableDumpMsg_PeerTableEntry::has_key,
                    PeerTableDumpMsg_PeerTableEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    PeerTableDumpMsg_PeerTableEntry::has_value,
                    PeerTableDumpMsg_PeerTableEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerTableDumpMsg_PeerTableEntry>(
                    "PeerTableDumpMsg_PeerTableEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerTableDumpMsg_PeerTableEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PeerTableDumpMsg_PeerTableEntry {
    fn eq(&self, other: &PeerTableDumpMsg_PeerTableEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerTableDumpMsg_PeerTableEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CoterieMsg {
    // message fields
    field_type: ::std::option::Option<CoterieMsg_Type>,
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
        self.field_type.unwrap_or(CoterieMsg_Type::OPEN_WRITE_STREAM)
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CoterieMsg {
    fn eq(&self, other: &CoterieMsg) -> bool {
        self.field_type == other.field_type &&
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
    OPEN_WRITE_STREAM = 0,
}

impl ::protobuf::ProtobufEnum for CoterieMsg_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CoterieMsg_Type> {
        match value {
            0 => ::std::option::Option::Some(CoterieMsg_Type::OPEN_WRITE_STREAM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CoterieMsg_Type] = &[
            CoterieMsg_Type::OPEN_WRITE_STREAM,
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
pub struct OpenWriteStreamMsg {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpenWriteStreamMsg {
    pub fn new() -> OpenWriteStreamMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpenWriteStreamMsg {
        static mut instance: ::protobuf::lazy::Lazy<OpenWriteStreamMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpenWriteStreamMsg,
        };
        unsafe {
            instance.get(|| {
                OpenWriteStreamMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for OpenWriteStreamMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<OpenWriteStreamMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpenWriteStreamMsg {
    fn new() -> OpenWriteStreamMsg {
        OpenWriteStreamMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpenWriteStreamMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<OpenWriteStreamMsg>(
                    "OpenWriteStreamMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpenWriteStreamMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpenWriteStreamMsg {
    fn eq(&self, other: &OpenWriteStreamMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpenWriteStreamMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfb, 0x01, 0x0a, 0x06, 0x44, 0x48, 0x54, 0x4d, 0x73, 0x67,
    0x12, 0x20, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0c,
    0x2e, 0x44, 0x48, 0x54, 0x4d, 0x73, 0x67, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x12, 0x32, 0x0a, 0x0d, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x5f,
    0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x48, 0x65, 0x61, 0x72,
    0x74, 0x62, 0x65, 0x61, 0x74, 0x4d, 0x73, 0x67, 0x52, 0x0c, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x23, 0x0a, 0x08, 0x6a, 0x6f, 0x69, 0x6e, 0x5f, 0x6d,
    0x73, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x4d,
    0x73, 0x67, 0x52, 0x07, 0x6a, 0x6f, 0x69, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x40, 0x0a, 0x13, 0x70,
    0x65, 0x65, 0x72, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x64, 0x75, 0x6d, 0x70, 0x5f, 0x6d,
    0x73, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x44, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x52, 0x10, 0x70, 0x65, 0x65,
    0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x22, 0x34, 0x0a,
    0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x48, 0x45, 0x41, 0x52, 0x54, 0x42, 0x45,
    0x41, 0x54, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x4a, 0x4f, 0x49, 0x4e, 0x10, 0x01, 0x12, 0x13,
    0x0a, 0x0f, 0x50, 0x45, 0x45, 0x52, 0x5f, 0x54, 0x41, 0x42, 0x4c, 0x45, 0x5f, 0x44, 0x55, 0x4d,
    0x50, 0x10, 0x02, 0x22, 0x0e, 0x0a, 0x0c, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74,
    0x4d, 0x73, 0x67, 0x22, 0x3b, 0x0a, 0x07, 0x4a, 0x6f, 0x69, 0x6e, 0x4d, 0x73, 0x67, 0x12, 0x16,
    0x0a, 0x06, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x03, 0x52, 0x06,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x22, 0x91, 0x01, 0x0a, 0x10, 0x50, 0x65, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x75,
    0x6d, 0x70, 0x4d, 0x73, 0x67, 0x12, 0x3f, 0x0a, 0x0a, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x50, 0x65, 0x65, 0x72,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x75, 0x6d, 0x70, 0x4d, 0x73, 0x67, 0x2e, 0x50, 0x65, 0x65,
    0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x09, 0x70, 0x65, 0x65,
    0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x1a, 0x3c, 0x0a, 0x0e, 0x50, 0x65, 0x65, 0x72, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x3a, 0x02, 0x38, 0x01, 0x22, 0x51, 0x0a, 0x0a, 0x43, 0x6f, 0x74, 0x65, 0x72, 0x69, 0x65, 0x4d,
    0x73, 0x67, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x10, 0x2e, 0x43, 0x6f, 0x74, 0x65, 0x72, 0x69, 0x65, 0x4d, 0x73, 0x67, 0x2e, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x1d, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x15, 0x0a, 0x11, 0x4f, 0x50, 0x45, 0x4e, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x5f, 0x53,
    0x54, 0x52, 0x45, 0x41, 0x4d, 0x10, 0x00, 0x22, 0x14, 0x0a, 0x12, 0x4f, 0x70, 0x65, 0x6e, 0x57,
    0x72, 0x69, 0x74, 0x65, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x42, 0x02, 0x48,
    0x01, 0x4a, 0x97, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x01, 0x00, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x01, 0x00, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x01, 0x16, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x03, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x04, 0x08, 0x08, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x06, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x07, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x0a, 0x08, 0x08, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x0a, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x14, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0b, 0x08, 0x0a, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0b, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x0b, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x0b, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c,
    0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x0c, 0x08, 0x0b,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0c, 0x08, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x10, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x0d, 0x08, 0x0c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x0d, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0d, 0x19, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d, 0x2f,
    0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x13, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x14, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x15, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x15, 0x08,
    0x14, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x10, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x18, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x19, 0x08,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x19, 0x08, 0x18, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x1a, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x1c, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1c,
    0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x1d, 0x08, 0x1f, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x0d, 0x11, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x10, 0x26, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x10, 0x21, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1e, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x21, 0x08, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x21, 0x08, 0x1f, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x21, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x21, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x21, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x24, 0x00, 0x25, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x24, 0x08, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
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
