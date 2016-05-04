#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate protobuf;

pub mod dht;
pub mod message;
pub mod parser;

use std::net::TcpStream;

use message::{Entity,Entity_EntityEntry,CoterieMsg,CoterieMsg_Type,ResultMsg,WriteEntitiesMsg};

use protobuf::{CodedInputStream,CodedOutputStream,RepeatedField};

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

pub fn write_coterie_msg(msg: &CoterieMsg, stream: &mut TcpStream) -> Result<(),String> {
    let mut coded_output_stream = CodedOutputStream::new(stream);
    coded_output_stream.write_message_no_tag(msg).ok().expect("unable to write coterie msg to stream");
    coded_output_stream.flush().ok().expect("unable to flush coded output stream");

    Ok(())
}

pub fn read_coterie_msg(stream: &mut TcpStream) -> Result<CoterieMsg,String> {
    let mut coded_input_stream = CodedInputStream::new(stream);
    let coterie_msg: CoterieMsg = coded_input_stream.read_message().ok().expect("unable to read coterie msg from stream`");

    Ok(coterie_msg)
}

pub fn create_close_write_stream_msg() -> CoterieMsg {
    let mut msg = CoterieMsg::new();
    msg.set_field_type(CoterieMsg_Type::CLOSE_WRITE_STREAM);
    msg
}

pub fn create_open_write_stream_msg() -> CoterieMsg {
    let mut msg = CoterieMsg::new();
    msg.set_field_type(CoterieMsg_Type::OPEN_WRITE_STREAM);
    msg
}

pub fn create_result_msg(success: bool, error_message: String) -> CoterieMsg {
    let mut result_msg = ResultMsg::new();
    result_msg.set_success(success);
    result_msg.set_error_message(error_message);

    let mut msg = CoterieMsg::new();
    msg.set_field_type(CoterieMsg_Type::RESULT);
    msg.set_result_msg(result_msg);
    msg
}

pub fn create_write_entities_msg(header: &Vec<String>, records: &Vec<Vec<String>>) -> CoterieMsg {
    let mut entities: RepeatedField<Entity> = RepeatedField::new();
    for record in records {
        let mut entity = Entity::new();
        let mut entries = RepeatedField::new();
        for (i, key) in header.iter().enumerate() {
            let mut entry = Entity_EntityEntry::new();
            entry.set_key(key.clone());
            entry.set_value(record[i].clone());
            entries.push(entry);
        }

        entity.set_entity(entries);
        entities.push(entity);
    }

    let mut write_entities_msg = WriteEntitiesMsg::new();
    write_entities_msg.set_entity(entities);

    let mut msg = CoterieMsg::new();
    msg.set_field_type(CoterieMsg_Type::WRITE_ENTITIES);
    msg.set_write_entities_msg(write_entities_msg);
    msg
}

pub fn parse_write_entities_msg(msg: &mut CoterieMsg) -> (Vec<String>, Vec<Vec<String>>) {
    let write_entities_msg = msg.get_write_entities_msg();
    let mut header = Vec::new();
    let mut records = Vec::new();
    for entity in write_entities_msg.get_entity().iter() {
        let mut record = Vec::new();
        for (i, entry) in entity.get_entity().iter().enumerate() {
            if header.len() < i {
                header.push(entry.get_key().to_string());
            }

            record.push(entry.get_value().to_string());
        }

        records.push(record);
    }

    (header, records)
}
