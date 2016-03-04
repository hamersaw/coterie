use std::collections::{BTreeMap,HashMap};
use std::io::{Read,Write};
use std::net::{SocketAddr,TcpListener,TcpStream};
use std::str::FromStr;
use std::sync::{Arc,RwLock};
use std::thread;

use message::{DHTMsg,DHTMsg_Type,JoinMsg};

use protobuf::{CodedInputStream,CodedOutputStream};

pub struct DHTService {
    tokens: Vec<i64>,
    app_addr: SocketAddr,
    dht_addr: SocketAddr,
    seeds: Vec<SocketAddr>,
    lookup_table: Arc<RwLock<BTreeMap<i64,SocketAddr>>>,
    dht_peer_table: Arc<RwLock<HashMap<SocketAddr,Vec<i64>>>>,
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
            dht_peer_table: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start(dht_service: Arc<RwLock<DHTService>>) {
        let listener: TcpListener;
        {
            let dht_service = dht_service.read().ok().expect("unable to get read lock on dht service");
            info!("starting the dht on address {}", dht_service.dht_addr);
            listener = TcpListener::bind(dht_service.dht_addr).ok().expect("unable to bind to dht address");
        }

        let dht_service_clone = dht_service.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let dht_service = dht_service_clone.clone();
                thread::spawn(move || {
                    let mut stream = stream.ok().expect("unable to unwrap TcpStream");
                    debug!("recv dht message");

                    let mut msg = read_dht_msg(&mut stream).ok().expect("unable to read dht msg from TcpStream");
                    match msg.get_field_type() {
                        DHTMsg_Type::JOIN => {
                            debug!("recv join msg");
                            let (tokens, app_addr) = parse_join_msg(&mut msg);
                            let mut dht_service = dht_service.write().ok().expect("unable to get write lock on dht service");

                            //add dht peer
                            let dht_addr = stream.peer_addr().ok().expect("unable to retrieve peer addr from stream");
                            dht_service.add_dht_peer(dht_addr, tokens.clone());
 
                            //add tokens to lookup table
                            for token in tokens {
                                dht_service.add_token(token, app_addr);
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
            debug!("Sending JoinMsg to seed:{}", seed_addr);
            dht_service.send_join_msg(seed_addr).ok().expect("unable to send join message");
        }
    }

    fn lookup(&self, token: i64) -> SocketAddr {
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

    fn send_join_msg(&self, addr: SocketAddr) -> Result<(),String> {
        let join_msg = create_join_msg(self.tokens.clone(), format!("{}", self.app_addr));
        let mut stream = TcpStream::connect(addr).ok().expect("unable to connect to tcp stream to send join msg");
        write_dht_msg(&join_msg, &mut stream).ok().expect("unable to write join message to tcp stream");

        Ok(())
    }

    fn add_token(&mut self, token: i64, addr: SocketAddr) {
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.entry(token).or_insert(addr);
    }

    fn remove_token(&mut self, token: &i64) {
        let mut lookup_table = self.lookup_table.write().unwrap();
        lookup_table.remove(token);
    }

    fn add_dht_peer(&mut self, addr: SocketAddr, tokens: Vec<i64>) {
        let mut dht_peer_table = self.dht_peer_table.write().unwrap();
        dht_peer_table.entry(addr).or_insert(tokens);
    }

    fn remove_dht_peer(&mut self, addr: &SocketAddr) {
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

fn create_join_msg(tokens: Vec<i64>, app_addr: String) -> DHTMsg {
    let mut join_msg = JoinMsg::new();
    join_msg.set_tokens(tokens);
    join_msg.set_address(app_addr);

    let mut msg = DHTMsg::new();
    msg.set_field_type(DHTMsg_Type::JOIN);
    msg.set_join_msg(join_msg);
    msg
}

fn parse_join_msg(msg: &mut DHTMsg) -> (Vec<i64>,SocketAddr) {
    let mut join_msg = msg.mut_join_msg();
    let tokens = join_msg.take_tokens();
    let addr = SocketAddr::from_str(join_msg.get_address()).ok().expect("unable to parse socket address form join message");

    (tokens, addr)
}
