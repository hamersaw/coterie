extern crate env_logger;
#[macro_use] extern crate log;
extern crate coterie;
extern crate protobuf;
extern crate toml;

use std::env;
use std::collections::HashMap;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::Read;
use std::fs::File;
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread::{JoinHandle, self};

use coterie::{create_result_msg, create_write_entity_msg, open_write_stream, parse_write_entities_msg, read_coterie_msg, write_coterie_msg};
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
    {
        let dht_service = dht_service.clone();
        DHTService::start(dht_service);
    }

    //start coterie service
    let coterie_service = Arc::new(RwLock::new(CoterieService::new(app_addr)));
    let handle = CoterieService::start(coterie_service, dht_service);
    let _ = handle.join();
}

struct CoterieService {
    addr: SocketAddr,
    entities: Arc<RwLock<HashMap<i64,HashMap<String,String>>>>,
    fields: Arc<RwLock<HashMap<String,HashMap<String,Vec<i64>>>>>,
}

impl CoterieService {
    pub fn new(addr: SocketAddr) -> CoterieService {
        CoterieService {
            addr: addr,
            entities: Arc::new(RwLock::new(HashMap::new())),
            fields: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start(coterie_service: Arc<RwLock<CoterieService>>, dht_service: Arc<RwLock<DHTService>>) -> JoinHandle<()> {
        let listener: TcpListener;
        {
            let coterie_service = coterie_service.read().ok().expect("unable to get read lock on coterie service");
            info!("starting the coterie on address {}", coterie_service.addr);
            listener = TcpListener::bind(coterie_service.addr).ok().expect("unable to bind to coterie address");
        }

        let (coterie_service, dht_service) = (coterie_service.clone(), dht_service.clone());
        thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.ok().expect("Unable to unwrap TcpStream");
                let (coterie_service, dht_service) = (coterie_service.clone(), dht_service.clone());
                thread::spawn(move || {
                    //handle streams
                    handle_stream(&mut stream, coterie_service, dht_service).ok().expect("error while handleing stream");
                    stream.shutdown(Shutdown::Both).ok().expect("unable to close coterie stream");
                });
            }
        })
    }

    fn add_entity(token: u64, entity: HashMap<String,String>) {
        unimplemented!();
    }
}

fn handle_stream(mut stream: &mut TcpStream, coterie_service: Arc<RwLock<CoterieService>>, dht_service: Arc<RwLock<DHTService>>) -> Result<(),&str> {
    let mut msg = read_coterie_msg(&mut stream).ok().expect("unable to read coterie msg from tcp stream");
    match msg.get_field_type() {
        CoterieMsg_Type::OPEN_WRITE_STREAM => {
            loop {
                let mut msg = read_coterie_msg(&mut stream).ok().expect("unable to read coterie message from open write stream");
                match msg.get_field_type() {
                    CoterieMsg_Type::WRITE_ENTITIES => {
                        println!("TODO write entities msg");
                        let entities = parse_write_entities_msg(&mut msg);
                        
                        let mut streams = HashMap::new();
                        for entity in entities.iter() {
                            //compute entity key
                            let mut hasher = SipHasher::new();
                            for value in entity.values() {
                                value.hash(&mut hasher);
                            }
                            let entity_key = hasher.finish() as i64;

                            {
                                //lookup into dht
                                let dht_service = dht_service.read().ok().expect("unable to get read lock on dht service");
                                let socket_addr = dht_service.lookup(entity_key);

                                //send write entity msg
                                let mut stream = streams.entry(socket_addr).or_insert_with(|| {open_write_stream(socket_addr)});
                                let write_entity_msg = create_write_entity_msg(&entity);
                                write_coterie_msg(&write_entity_msg, &mut stream).ok().expect("unable to send write entity message");
                            }

                            //TODO hash individual fields and write values
                        }

                        let result_msg = create_result_msg(true, format!(""));
                        write_coterie_msg(&result_msg, &mut stream).ok().expect("unable to write result message from write entities");
                    },
                    CoterieMsg_Type::WRITE_ENTITY => {
                        println!("TODO write entity msg");
                    },
                    CoterieMsg_Type::CLOSE_WRITE_STREAM => break,
                    _ => panic!("unexpected msg type over write stream"),
                }
            }
        },
        _ => panic!("unimplemented coterie msg type")
    }

    Ok(())
}
