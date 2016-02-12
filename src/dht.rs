use std::collections::{BTreeMap,HashMap};
use std::net::{SocketAddrV4,TcpListener};
use std::sync::{Arc,RwLock};
use std::thread;

pub struct DHTService {
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
        tokens.into_iter().map(|x| { lookup_table.insert(x, app_addr) });

        DHTService {
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
