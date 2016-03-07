extern crate argparse;
extern crate coterie;
extern crate nom;

use std::io;
use std::io::prelude::*; //needed for flushing stdout
use std::net::{SocketAddr,TcpStream};
use std::str::FromStr;

use coterie::parser::Command::{Exit,Help,Load,Query};

use argparse::{ArgumentParser,Store};

static HELP_MSG: &'static str = 
"   EXIT => exit the session
    HELP => print this menu
    LOAD <filename> => load csv file into cluster
    SELECT [ * | <field> ( , <field> )* ] WHERE <field> ~<type> <value> (AND <field> ~<type> <value>)* => perfrom query on cluster";

fn main() {
    let mut host_ip: String = "127.0.0.1".to_string();
    let mut host_port: u16 = 15605;
    let mut batch_size: u16 = 250;
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
                println!("TODO - load {}", filename);
            },
            Query(_, _) => {
                println!("TODO - execute query");
            },
        }
    }
}
