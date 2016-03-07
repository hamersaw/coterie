extern crate env_logger;
#[macro_use] extern crate log;
extern crate coterie;
extern crate protobuf;
extern crate toml;

use std::env;
use std::io::Read;
use std::fs::File;
use std::net::{SocketAddr,TcpListener};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::thread;

use coterie::{read_coterie_msg,write_coterie_msg};
use coterie::dht::DHTService;
use coterie::message::CoterieMsg_Type;

use toml::Value::Table;

pub fn main() {
    env_logger::init().unwrap();

    //read toml configuration file
    let mut args = env::args();
    if args.len() < 1 {
        error!("Missing configuration filename");
        return
    }

    let mut input = String::new();
    let filename = args.nth(1).unwrap();
    File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let mut parser = toml::Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                error!("{}:{}:{}-{}:{} error:{}", filename, loline, locol, hiline, hicol, err.desc)
            }
            return
        },
    };

    //parse toml
    let table = Table(toml);
    let tokens: Vec<i64> = table.lookup("tokens").expect("Unable to parse token from toml configuration")
        .as_slice().expect("Unable to parse tokens as slice")
        .into_iter()
        .map(|x| {
            x.as_integer().expect("Unable to parse token as int")
        }).collect();

    let app_ip_address = table.lookup("application.ip_address").expect("Unable to parse application.ip_address from toml configuration").as_str().unwrap();
    let app_port = table.lookup("application.port").expect("Unable to parse application.port from toml configuration").as_integer().expect("Unable to parse application.port as integer");
    let app_addr = SocketAddr::from_str(&format!("{}:{}", app_ip_address, app_port)).ok().expect("unable to parse application address into socket address");

    let dht_ip_address = table.lookup("dht.ip_address").expect("Unable to parse dht.ip_address from toml configuration").as_str().unwrap();
    let dht_port = table.lookup("dht.port").expect("Unable to parse dht.port from toml configuration").as_integer().expect("Unable to parse dht.port as integer");
    let dht_addr = SocketAddr::from_str(&format!("{}:{}", dht_ip_address, dht_port)).ok().expect("unable to parse dht address into socket address");

    let seeds: Vec<SocketAddr> = match table.lookup("seeds") {
        Some(value) => {
            value.as_slice().expect("Unable to parse seeds as an array").into_iter()
                .map(|x| {
                    let map = Table(x.as_table().expect("Unable to parse seed as map").clone());
                    let ip_address = map.lookup("ip_address").expect("Unable to parse seeds.ip_address from toml configuration").as_str().unwrap();
                    let port = map.lookup("port").expect("Unable to parse seeds.port from toml configuration").as_integer().expect("Unable to parse seeds.port as integer");

                    SocketAddr::from_str(&format!("{}:{}", ip_address, port)).ok().expect("unable to parse seed address into socket address")
                }).collect()
        },
        None => vec!(),
    };

    //start the dht
    let dht_service = Arc::new(RwLock::new(DHTService::new(tokens, app_addr, dht_addr, seeds)));
    DHTService::start(dht_service);

    //start the coterie listener
    info!("starting the application with ip_address:{} port:{}", app_ip_address, app_port);
    let listener = TcpListener::bind(app_addr).ok().expect("Unable to bind to address");
    for stream in listener.incoming() {
        let mut stream = stream.ok().expect("Unable to unwrap TcpStream");
        thread::spawn(move || {
            let mut msg = read_coterie_msg(&mut stream).ok().expect("unable to read dht msg from tcp stream");
            match msg.get_field_type() {
                CoterieMsg_Type::OPEN_WRITE_STREAM => {
                    println!("handle openw write stream msg");
                },
                //_ => panic!("unimplemented coterie msg type")
            }
        });
    }
}
