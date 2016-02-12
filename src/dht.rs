use std::collections::{BTreeMap,HashMap};
use std::net::{SocketAddrV4,TcpListener};
use std::sync::{Arc,RwLock};
use std::thread;

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
            lookup_table.insert(token, app_addr).unwrap();
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
            self.send_join_msg(seed_addr).unwrap();
        }

        let listener = TcpListener::bind(self.dht_addr).ok().expect("Unable to bind to address");
        let (lookup_table, dht_peer_table) = (self.lookup_table.clone(), self.dht_peer_table.clone());
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.ok().expect("Unable to unwrap TcpStream");
                let (lookup_table, dht_peer_table) = (lookup_table.clone(), dht_peer_table.clone());
                thread::spawn(move || {
                    //TODO read and process messages
                });
            }
        });
    }

    pub fn lookup(&self, token: i64) -> SocketAddrV4 {
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
        let join_msg = create_join_msg(&self.tokens, &self.app_addr);

        //TODO open up a stream and send the join
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

fn create_join_msg(tokens: &Vec<i64>, addr: &SocketAddrV4) -> DHTMsg {
    let socket_addr = create_socket_addr(addr);
    let mut join_msg = JoinMsg::new();
    join_msg.set_tokens(tokens.clone());
    join_msg.set_address(socket_addr);

    let mut dht_msg = DHTMsg::new();
    dht_msg.set_field_type(DHTMsg_Type::JOIN);
    dht_msg.set_join_msg(join_msg);
    dht_msg
}

fn create_socket_addr(addr: &SocketAddrV4) -> SocketAddr {
    let mut socket_addr = SocketAddr::new();
    socket_addr.set_ip_address(format!("{}", addr.ip()));
    socket_addr.set_port(addr.port() as u32);

    socket_addr
}
