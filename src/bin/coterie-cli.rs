extern crate argparse;
extern crate coterie;
extern crate csv;
extern crate nom;

use std::io;
use std::io::prelude::*; //needed for flushing stdout
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::str::FromStr;

use coterie::{create_close_write_stream_msg, create_write_entities_msg, open_write_stream, read_coterie_msg, write_coterie_msg};
use coterie::parser::Command::{Exit, Help, Load, Query};
use coterie::message::CoterieMsg_Type;

use argparse::{ArgumentParser,Store};

static HELP_MSG: &'static str = 
"   EXIT => exit the session
    HELP => print this menu
    LOAD <filename> => load csv file into cluster
    SELECT [ * | <field> ( , <field> )* ] WHERE <field> ~<type> <value> (AND <field> ~<type> <value>)* => perfrom query on cluster";

fn main() {
    let mut host_ip: String = "127.0.0.1".to_string();
    let mut host_port: u16 = 15605;
    let mut batch_size: usize = 250;
    {    //solely to limit scope of parser variable
        let mut parser = ArgumentParser::new();
        parser.set_description("start a coterie client session");
        parser.refer(&mut host_ip).add_option(&["-i", "--host-ip"], Store, "IP address of the host to connect to");
        parser.refer(&mut host_port).add_option(&["-p", "--host-port"], Store, "Port of the host to connect to");
        parser.refer(&mut batch_size).add_option(&["-s", "--batch-size"], Store, "Number of records in each batch sent for insertion");
        parser.parse_args_or_exit();
    }

    //parse the host address
    let host_addr = SocketAddr::from_str(&format!("{}:{}", host_ip, host_port)).ok().expect("unable to parse address into socket address");

    //loop read user input
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print!("Enter input: ");
        std::io::stdout().flush().ok(); //future versions of rust will fix this need

        line.clear();
        stdin.read_line(&mut line).ok();
        line = line.trim().to_string();

        //parse command
        let cmd = match coterie::parser::cmd(&line.clone().into_bytes()[..]) {
            nom::IResult::Done(bytes, cmd) => {
                if bytes.len() != 0 {
                    Help
                } else {
                    cmd
                }
            },
            _ => {
                println!("Invalid input command");
                Help
            },
        };

        //execute command
        match cmd {
            Exit => break,
            Help => println!("{}", HELP_MSG),
            Load(filename) => {
                //open csv file reader
                let reader = csv::Reader::from_file(filename.clone());
                if !reader.is_ok() {
                    println!("file '{}' does not exist or cannot be opened", filename);
                    continue;
                }
                let mut reader = reader.unwrap();

                //read csv file header
                let header = reader.headers().unwrap();

                //send open write stream msg
                //let open_write_stream_msg = create_open_write_stream_msg();
                //let mut stream = TcpStream::connect(host_addr).ok().expect("unable to connect to coterie server");
                //write_coterie_msg(&open_write_stream_msg, &mut stream).ok().expect("unable to write open write stream msg");
                let mut stream = open_write_stream(host_addr);

                //loop through records
                let mut record_count = 0;
                let mut record_buffer = Vec::new();
                for record in reader.records() {
                    let record = record.unwrap();
                    record_buffer.push(record);
                    record_count += 1;

                    if record_buffer.len() == batch_size {
                        let write_entities_msg = create_write_entities_msg(&header, &record_buffer);
                        write_coterie_msg(&write_entities_msg, &mut stream).ok().expect("unable to write write entities msg");

                        let msg = read_coterie_msg(&mut stream).ok().expect("unable to read coterie message");
                        match msg.get_field_type() {
                            CoterieMsg_Type::RESULT => {
                                
                            },
                            _ => panic!("unexpected return message type"),
                        }
                        record_buffer.clear();
                    }
                }

                if record_buffer.len() != 0 {
                    let write_entities_msg = create_write_entities_msg(&header, &record_buffer);
                    write_coterie_msg(&write_entities_msg, &mut stream).ok().expect("unable to write write entities msg");
                }

                //send close write stream msg
                let close_write_stream_msg = create_close_write_stream_msg();
                write_coterie_msg(&close_write_stream_msg, &mut stream).ok().expect("unable to write close write stream msg");
                stream.shutdown(Shutdown::Both).ok().expect("unable to close write stream");

                println!("total record count:{}", record_count);
            },
            Query(_, _) => {
                println!("TODO - execute query");
            },
        }
    }
}
