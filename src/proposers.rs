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
    status: Status,

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

    pub fn status(&self) -> &Status{
        &self.status
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
        // Check status of proposer
        if self.status == Status::Active && self.val > 0{
            // If status is Active and value > 0 is set, send propose messages out
            for acc in list.iter(){
                if acc.status() != &Status::Promised 
                && acc.status() != &Status::Accepted 
                && acc.status() != &Status::Failed {
                    self.to_send.insert(acc.id(), Message::Prepare(self.id, self.id));
                }
            }
        }
        else {
            // Otherwise, deal with messages in the message queue and perform actions based on them 
            self.check_buffer(buffer);
        }
        self.send_buffer(buffer);
            
    }

    pub fn check_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>) {
        println!("Checking Buffer");
        if buffer.contains_key(&self.id) && self.status != Status::Failed {
            let bucket = buffer.get_mut(&self.id).unwrap();
            while bucket.len() > 0 {
                let message_tup = bucket.pop().unwrap();
                // Analyse the received message
                // Reply with a message to acceptors is needed
            }
        }
    }

    pub fn send_buffer(&mut self, buffer: &mut HashMap<u32, Vec<Message>>){
        println!("Sending to Buffer");
        if self.status != Status::Failed {
            for (k,v) in self.to_send.drain(){
                // If the messaged is Accepted by over half acceptors then declare accepted value
                if buffer.contains_key(&k){
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
            status: Status::Active
        }
    }
}

impl fmt::Display for Proposer{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proposer => num_acceptors: {}, id: {}, val: {}, status: {}", self.num_acceptors, self.id, self.val, self.status)
    }

}