#[macro_use] extern crate log;
#[macro_use] extern crate nom;
extern crate protobuf;

pub mod dht;
pub mod message;
pub mod parser;

use std::net::TcpStream;

use message::CoterieMsg;

use protobuf::{CodedInputStream,CodedOutputStream};

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
