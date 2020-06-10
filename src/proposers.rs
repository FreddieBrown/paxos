use std::fmt;
use crate::messages::Message;
use crate::messages::Status;
use crate::acceptors::Acceptor;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Proposer{
    id: u32,
    val: u32,
    num_acceptors: u32,
    to_send: HashMap<u32, Message>,
    promised: HashSet<u32>,
    accepted: HashSet<u32>,
    pub status: Status,
    prom_state: bool

}

impl Proposer{

    pub fn num_acceptors(&self) -> u32{
        self.num_acceptors
    }

    pub fn id(&self) -> u32{
        self.id
    }

    pub fn val(&self) -> u32{
        self.val
    }

    pub fn set_num_acceptors(&mut self, num: u32){
        self.num_acceptors = num;
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

    pub fn set_val(&mut self, val: u32){
        self.val = val;
    }

    pub fn set_status(&mut self, status: Status){
        self.status = status;
    }

    pub fn run(&mut self, list: &Vec<Acceptor>, buffer: &mut HashMap<u32, Vec<Message>>){
        if self.status == Status::Active && self.val > 0{
            for acc in list.iter(){
                match acc.status {
                    Status::Active => {
                        self.status = Status::Prepared;
                        self.to_send.insert(acc.id(), Message::Prepare(self.id, self.id));
                    },
                    Status::Promised => {
                        self.status = Status::Prepared;
                        self.to_send.insert(acc.id(), Message::Prepare(self.id, self.id));
                    },
                    _ => println!("{}", &acc)
                };
            }
        }
        else {
            self.check_buffer(buffer);
        }
        self.send_buffer(buffer);
    }

    pub fn check_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>) {
        if buffer.contains_key(&self.id) && self.status != Status::Failed {
            let acc_size: usize = ((self.num_acceptors/2)+1) as usize;
            let bucket = buffer.get_mut(&self.id).unwrap();
            while bucket.len() > 0 {
                let message = bucket.pop().unwrap();
                match message {
                    Message::Promise(id,sid) => {
                        self.promised.insert(sid);
                        if self.prom_state {
                            self.to_send.insert(sid,Message::Propose(id, self.val, id));
                        }
                        else if self.promised.len() > acc_size {
                            self.status = Status::Promised;
                            self.prom_state = true;
                            for aid in self.promised.iter(){
                                self.to_send.insert(*aid,Message::Propose(id, self.val, id));
                            }
                        }
                        
                    },
                    Message::Accepted(_,_,sid) => {
                        self.accepted.insert(sid);
                        if self.accepted.len() > acc_size {
                            self.status = Status::Accepted;
                        }   
                    },
                    Message::Fail(_,_) => {
                        self.status = Status::Failed;
                    },
                    _ => ()
                };
            }
        }
    }

    pub fn send_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>){
        if self.status != Status::Failed {
            for (k,v) in self.to_send.drain(){
                // TODO: If the messaged is Accepted by over half acceptors then declare accepted value
                if buffer.contains_key(&k){
                    println!("Sending ({}) to Acc {} from Prop {}", &v, &k, &self.id);
                    let bucket = buffer.get_mut(&k).unwrap();
                    bucket.push(v);
                }
            }
        }
    }
}

impl Default for Proposer{
    fn default() -> Proposer{
        Proposer{
            id: 0,
            val: 0,
            num_acceptors: 0,
            to_send: HashMap::new(),
            promised: HashSet::new(),
            accepted: HashSet::new(),
            status: Status::Active,
            prom_state: false
        }
    }
}

impl fmt::Display for Proposer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proposer => num_acceptors: {}, id: {}, val: {}, status: {}", self.num_acceptors, self.id, self.val, self.status)
    }

}