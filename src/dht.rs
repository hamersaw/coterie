use std::collections::{BTreeMap,HashMap};
use std::io::{Read,Write};
use std::net::{Shutdown,SocketAddr,TcpListener,TcpStream};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::thread::{JoinHandle,self};

use message::{DHTMsg,DHTMsg_Type,JoinMsg,DumpMsg,DumpMsg_LookupTableEntry};

use protobuf::{CodedInputStream,CodedOutputStream,RepeatedField};

pub struct DHTService {
    tokens: Vec<i64>,
    app_addr: SocketAddr,
    dht_addr: SocketAddr,
    seeds: Vec<SocketAddr>,
    lookup_table: Arc<RwLock<BTreeMap<i64,SocketAddr>>>,
    peer_table: Arc<RwLock<HashMap<SocketAddr,Vec<i64>>>>,
}

impl DHTService {
    pub fn new(tokens: Vec<i64>, app_addr: SocketAddr, dht_addr: SocketAddr, seeds: Vec<SocketAddr>) -> DHTService {
        //create and populate initial lookup table
        let mut lookup_table = BTreeMap::new();
        for token in tokens.clone() {
            lookup_table.insert(token, app_addr);
        }

        DHTService {
            tokens: tokens,
            app_addr: app_addr,
            dht_addr: dht_addr,
            seeds: seeds,
            lookup_table: Arc::new(RwLock::new(lookup_table)),
            peer_table: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start(dht_service: Arc<RwLock<DHTService>>) -> JoinHandle<()> {
        let listener: TcpListener;
        {
            let dht_service = dht_service.read().ok().expect("unable to get read lock on dht service");
            info!("starting the dht on address {}", dht_service.dht_addr);
            listener = TcpListener::bind(dht_service.dht_addr).ok().expect("unable to bind to dht address");
        }

        let dht_service_clone = dht_service.clone();
        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let dht_service = dht_service_clone.clone();
                thread::spawn(move || {
                    let mut stream = stream.ok().expect("unable to unwrap TcpStream");
                    let mut msg = read_dht_msg(&mut stream).ok().expect("unable to read dht msg from TcpStream");

                    match msg.get_field_type() {
                        DHTMsg_Type::JOIN => {
                            debug!("recv join msg");
                            let (dht_addr, app_addr, tokens) = parse_join_msg(&mut msg);
                            let mut dht_service = dht_service.write().ok().expect("unable to get write lock on dht service");

                            if dht_service.contains_peer(&dht_addr) {
                                return;
                            }

                            //forward join msg to all other nodes in dht
                            {
                                let peer_table = dht_service.peer_table.read().ok().expect("unable to get read lock on dht peer table");
                                for addr in peer_table.keys() {
                                    let mut stream = TcpStream::connect(addr).ok().expect(&format!("unable to connect to address {}", addr));
                                    write_dht_msg(&msg, &mut stream).ok().expect("unable to forward join msg");
                                    stream.shutdown(Shutdown::Both).unwrap();
                                }
                            }

                            //add dht addr to peer table and tokens to lookup table
                            dht_service.add_peer(dht_addr, tokens.clone());
                            for token in tokens {
                                dht_service.add_token(token, app_addr);
                            }

                            //send peer table dump msg to joining node
                            let mut stream = TcpStream::connect(dht_addr).ok().expect(&format!("unable to connect to joining node address {}", dht_addr));
                            let peer_table_dump_msg = dht_service.create_dump_msg();
                            write_dht_msg(&peer_table_dump_msg, &mut stream).ok().expect("unable to send peer table dump msg");
                            stream.shutdown(Shutdown::Both).unwrap();
                        },
                        DHTMsg_Type::DUMP => {
                            debug!("recv lookup dump msg");
                            let (dht_addr,tokens,lookup_table) = parse_dump_msg(&mut msg);
                            let mut dht_service = dht_service.write().ok().expect("unable to get write lock on dht service");

                            //add dht addr to peer table
                            dht_service.add_peer(dht_addr, tokens);
                            for (token, addr) in lookup_table.iter() {
                                dht_service.add_token(*token, *addr);
                            }
                        },
                        _ => panic!("unimplemented dht msg type")
                    }
                });
            }
        });
    
        //sending join messages to all of the seeds
        let dht_service = dht_service.read().ok().expect("unable to get read lock on dht service");
        for seed_addr in dht_service.seeds.clone() {
            debug!("sending JoinMsg to seed:{}", seed_addr);
            let mut stream = TcpStream::connect(seed_addr).ok().expect(&format!("unable to connect to seed address {}", seed_addr));

            let join_msg = dht_service.create_join_msg();
            write_dht_msg(&join_msg, &mut stream).ok().expect("unable to send join msg");
        }

        handle
    }

    pub fn lookup(&self, token: i64) -> SocketAddr {
        debug!("lookup on token:{}", token);
        let lookup_table = self.lookup_table.read().unwrap();

        //get first (smallest) token from the peer table
        let mut iter = lookup_table.iter();
        let first_tuple = iter.next().unwrap();

        //if search token is smaller than first
        if token < *first_tuple.0 {
            *first_tuple.1;
        };

        //search in between every set of concurrent tokens
        let mut last_token = *first_tuple.0;
        for (current_token, socket_addr) in iter {
            if last_token < token && *current_token >= token {
                return *socket_addr;
            }

            last_token = *current_token;
        }

        *first_tuple.1
    }

    fn add_token(&mut self, token: i64, addr: SocketAddr) {
        trace!("adding token {} address {}", token, addr);
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.entry(token).or_insert(addr);
    }

    fn remove_token(&mut self, token: &i64) {
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.remove(token);
    }

    fn contains_peer(&mut self, addr: &SocketAddr) -> bool {
        let peer_table = self.peer_table.read().unwrap();
        peer_table.contains_key(addr)
    }

    fn add_peer(&mut self, addr: SocketAddr, tokens: Vec<i64>) {
        trace!("adding peer {}", addr);
        let mut peer_table = self.peer_table.write().unwrap();
        peer_table.entry(addr).or_insert(tokens);
    }

    fn remove_peer(&mut self, addr: &SocketAddr) {
        let mut peer_table = self.peer_table.write().unwrap();
        peer_table.remove(addr);
    }

    fn create_join_msg(&self) -> DHTMsg {
        let mut join_msg = JoinMsg::new();
        join_msg.set_dht_address(format!("{}", self.dht_addr));
        join_msg.set_application_address(format!("{}", self.app_addr));
        join_msg.set_tokens(self.tokens.clone());

        let mut msg = DHTMsg::new();
        msg.set_field_type(DHTMsg_Type::JOIN);
        msg.set_join_msg(join_msg);
        msg
    }

    fn create_dump_msg(&self) -> DHTMsg {
        let mut lookup_fields: RepeatedField<DumpMsg_LookupTableEntry> = RepeatedField::new();
        let lookup_table = self.lookup_table.read().unwrap();
        for (token, addr) in lookup_table.iter() {
            let mut field = DumpMsg_LookupTableEntry::new();
            field.set_key(*token);
            field.set_value(format!("{}", addr));
            lookup_fields.push(field);
        }

        let mut dump_msg = DumpMsg::new();
        dump_msg.set_dht_address(format!("{}", self.dht_addr));
        dump_msg.set_tokens(self.tokens.clone());
        dump_msg.set_lookup_table(lookup_fields);

        let mut msg = DHTMsg::new();
        msg.set_field_type(DHTMsg_Type::DUMP);
        msg.set_dump_msg(dump_msg);
        msg
    }
}

fn write_dht_msg(msg: &DHTMsg, stream: &mut TcpStream) -> Result<(),String> {
    let mut coded_output_stream = CodedOutputStream::new(stream);
    coded_output_stream.write_message_no_tag(msg).ok().expect("unable to write dht msg to stream");
    coded_output_stream.flush().ok().expect("unable to flush coded output stream");
 
    Ok(())
}

fn read_dht_msg(stream: &mut TcpStream) -> Result<DHTMsg,String> {
    let mut coded_input_stream = CodedInputStream::new(stream);
    let dht_msg: DHTMsg = coded_input_stream.read_message().ok().expect("unable to read dht msg from stream`");

    Ok(dht_msg)
}

fn parse_join_msg(msg: &mut DHTMsg) -> (SocketAddr,SocketAddr,Vec<i64>) {
    let mut join_msg = msg.mut_join_msg();
    let dht_addr = SocketAddr::from_str(join_msg.get_dht_address()).ok().expect("unable to parse dht address from join msg");
    let app_addr = SocketAddr::from_str(join_msg.get_application_address()).ok().expect("unable to parse application address from join msg");
    let tokens = join_msg.take_tokens();

    (dht_addr, app_addr, tokens)
}

fn parse_dump_msg(msg: &mut DHTMsg) -> (SocketAddr,Vec<i64>,HashMap<i64,SocketAddr>) {
    let mut dump_msg = msg.mut_dump_msg();
    let dht_addr = SocketAddr::from_str(dump_msg.get_dht_address()).ok().expect("unable to parse dht address from lookup dump msg");
    let tokens = dump_msg.take_tokens();

    let mut lookup_table = HashMap::new();
    for field in dump_msg.take_lookup_table().iter() {
        let addr = SocketAddr::from_str(&format!("{}", field.get_value())).ok().expect("unable to parse address into socket address from lookup dump msg");
        lookup_table.insert(field.get_key(), addr);
    }

    (dht_addr, tokens, lookup_table)
}
