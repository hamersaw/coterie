use std::collections::{BTreeMap,HashMap};
use std::io::{Read,Write};
use std::net::{SocketAddrV4,TcpListener,TcpStream};
use std::sync::{Arc,RwLock};
use std::thread;

use protobuf::{CodedInputStream,CodedOutputStream};
use message::{DHTMsg,DHTMsg_Type,JoinMsg,SocketAddr};

pub struct DHTService {
    tokens: Vec<i64>,
    app_addr: SocketAddrV4,
    dht_addr: SocketAddrV4,
    seeds: Vec<SocketAddrV4>,
    lookup_table: Arc<RwLock<BTreeMap<i64,SocketAddrV4>>>,
    dht_peer_table: Arc<RwLock<HashMap<SocketAddrV4,Vec<i64>>>>,
}

impl DHTService {
    pub fn new(tokens: Vec<i64>, app_addr: SocketAddrV4, dht_addr: SocketAddrV4, seeds: Vec<SocketAddrV4>) -> DHTService {
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
            dht_peer_table: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start(&self) {
        debug!("starting the dht on address {}", self.dht_addr);

        //sending join messages to all of the seeds
        for seed_addr in self.seeds.clone() {
            debug!("Sending JoinMsg to seed:{}", seed_addr);
            self.send_join_msg(seed_addr).ok().expect("unable to send join message");
        }

        let listener = TcpListener::bind(self.dht_addr).ok().expect("unable to bind to address");
        let (lookup_table, dht_peer_table) = (self.lookup_table.clone(), self.dht_peer_table.clone());
        thread::spawn(move || {
            for stream in listener.incoming() {
                let (lookup_table, dht_peer_table) = (lookup_table.clone(), dht_peer_table.clone());
                thread::spawn(move || {
                    let mut stream = stream.ok().expect("unable to unwrap TcpStream");
                    debug!("recv dht message");

                    let msg = read_dht_msg(&mut stream).ok().expect("unable to read dht msg from tcp stream");
                    match msg.get_field_type() {
                        DHTMsg_Type::JOIN => {
                            debug!("recv join msg");
                        },
                        _ => panic!("unimplemented dht msg type"),
                    }
                });
            }
        });
    }

    pub fn lookup(&self, token: i64) -> SocketAddrV4 {
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

    fn send_join_msg(&self, addr: SocketAddrV4) -> Result<(),String> {
        let join_msg = create_join_msg(&self.tokens, format!("{}", self.app_addr.ip()), self.app_addr.port() as u32);

        let mut stream = TcpStream::connect(addr).ok().expect("unable to connect to tcp stream to send join msg");
        write_dht_msg(&join_msg, &mut stream).ok().expect("unable to write join message to tcp stream");

        //TODO read response

        Ok(())
    }

    fn add_token(&mut self, token: i64, addr: SocketAddrV4) {
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.entry(token).or_insert(addr);
    }

    fn remove_token(&mut self, token: &i64) {
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.remove(token);
    }

    fn add_dht_peer(&mut self, addr: SocketAddrV4, tokens: Vec<i64>) {
        let mut dht_peer_table = self.dht_peer_table.write().unwrap();
        dht_peer_table.entry(addr).or_insert(tokens);
    }

    fn remove_dht_peer(&mut self, addr: &SocketAddrV4) {
        let mut dht_peer_table = self.dht_peer_table.write().unwrap();
        dht_peer_table.remove(addr);
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

fn create_join_msg(tokens: &Vec<i64>, ip: String, port: u32) -> DHTMsg {
    let socket_addr = create_socket_addr(ip, port);
    let mut join_msg = JoinMsg::new();
    join_msg.set_tokens(tokens.clone());
    join_msg.set_address(socket_addr);

    let mut dht_msg = DHTMsg::new();
    dht_msg.set_field_type(DHTMsg_Type::JOIN);
    dht_msg.set_join_msg(join_msg);
    dht_msg
}

fn create_socket_addr(ip_address: String, port: u32) -> SocketAddr {
    let mut socket_addr = SocketAddr::new();
    socket_addr.set_ip_address(ip_address);
    socket_addr.set_port(port);

    socket_addr
}
