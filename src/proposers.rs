use std::fmt;
use crate::messages::Message;
use crate::messages::Status;
use crate::acceptors::Acceptor;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Proposer{
    id: u32,
    val: u32,
    num_acceptors: u32,
    to_send: HashMap<u32, Message>,
    promised: HashSet<u32>,
    accepted: HashSet<u32>,
    aval: u32,
    aid: u32,
    pub status: Status,
    prom_state: bool

}

impl Proposer{
    
    /// Getter for num_acceptors
    ///
    /// # Returns
    ///
    /// u32 - Number of acceptors in the running.
    ///
    pub fn num_acceptors(&self) -> u32{
        self.num_acceptors
    }

    /// Getter for id
    ///
    /// # Returns
    ///
    /// u32 - Id for the proposer.
    ///
    pub fn id(&self) -> u32{
        self.id
    }
    
    /// Getter for value in the proposer
    ///
    /// # Returns
    ///
    /// u32 - Value that is contained in proposer.
    ///
    pub fn val(&self) -> u32{
        self.val
    }

    /// Setter for num_acceptors
    ///
    /// # Arguments
    /// 
    /// * `num` - The number of acceptors running.
    ///
    pub fn set_num_acceptors(&mut self, num: u32){
        self.num_acceptors = num;
    }
    
    /// Setter for id
    ///
    /// # Arguments
    /// 
    /// * `id` - Id of the Proposer.
    ///
    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }
    
    /// Setter for val
    ///
    /// # Arguments
    /// 
    /// * `val` - Value to store in the Proposer, which it will share.
    ///
    pub fn set_val(&mut self, val: u32){
        self.val = val;
    }
    
    /// Setter for status
    ///
    /// # Arguments
    /// 
    /// * `status` - The status of the Proposer. Lets other nodes know what stage it is in,
    ///
    pub fn set_status(&mut self, status: Status){
        self.status = status;
    }

    /// Function to perform actions needed to respond to Promise messages.
    ///
    /// # Arguments
    /// 
    /// * `id` - Id from the message.
    /// * `sid` - Sender Id, usually the Id from the acceptor which sent a Promise message
    /// * `acc_size` - Number of Acceptors which need to sent Promise messages before Proposer will reply 
    ///
    fn promised_sub(&mut self, id: u32, sid: u32, acc_size: usize){
        if self.prom_state {
            self.to_send.insert(sid,Message::Propose(id, self.val, id));
        }
        else if self.promised.len() > acc_size {
            if self.aval > 0 {
                self.val = self.aval;
            }
            self.status = Status::Promised;
            self.prom_state = true;
            for aid in self.promised.iter(){
                self.to_send.insert(*aid,Message::Propose(id, self.val, id));
            }
        }
    }
    
    /// Function to perform proposer actions.
    ///
    /// # Arguments
    /// 
    /// * `list` - List of running Acceptors
    /// * `buffer` - Shared buffer where messages are shared between nodes in the network
    ///
    pub fn run(&mut self, list: &Arc<Vec<Mutex<Acceptor>>>, buffer: &Arc<HashMap<u32, Mutex<Vec<Message>>>>){
        if self.status == Status::Active && self.val > 0{
            for acc_mut in (*list).iter(){
                let acc = acc_mut.lock().unwrap();
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

    /// Function to check the buffer of the Proposer. It will check for any messages that are
    /// intended for the Proposer and will determine what the reply should be, if one is needed 
    /// and it will transition the proposer between states.
    ///
    /// # Arguments
    /// 
    /// * `buffer` - Shared buffer where messages are shared between nodes in the network
    ///
    pub fn check_buffer(&mut self, buffer: &Arc<HashMap<u32, Mutex<Vec<Message>>>>) {
        if (*buffer).contains_key(&self.id) && self.status != Status::Failed {
            let acc_size: usize = ((self.num_acceptors/2)+1) as usize;
            let mut bucket = (*buffer).get(&self.id).unwrap().lock().unwrap();
            while bucket.len() > 0 {
                let message = bucket.pop().unwrap();
                match message {
                    // Add an option for ACC_PROM
                    Message::Promise(id,sid) => {
                        self.promised.insert(sid);
                        self.promised_sub(id, sid, acc_size);
                    },
                    Message::AcceptedPromise(id, aid, aval, sid) => {
                        if aid > self.aid{
                            self.aval = aval;
                        }
                        self.promised.insert(sid);
                        self.promised_sub(id, sid, acc_size);
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

    /// Function to take messages from the out queue (`to_send`) and will place them in the 
    /// correct buckets in the shared message buffer
    ///
    /// # Arguments
    /// 
    /// * `buffer` - Shared buffer where messages are shared between nodes in the network
    ///
    pub fn send_buffer(&mut self, buffer: &Arc<HashMap<u32, Mutex<Vec<Message>>>>){
        if self.status != Status::Failed {
            for (k,v) in self.to_send.drain(){
                if (*buffer).contains_key(&k){
                    println!("Sending ({}) to Acc {} from Prop {}", &v, &k, &self.id);
                    let mut bucket = (*buffer).get(&k).unwrap().lock().unwrap();
                    bucket.push(v);
                }
            }
        }
    }
}

impl Default for Proposer{

    /// Default function for Proposer which will generate a basic 
    /// Proposer object and return it
    ///
    /// # Returns
    ///
    /// Proposer with all variables set to default values.
    ///
    fn default() -> Proposer{
        Proposer{
            id: 0,
            val: 0,
            num_acceptors: 0,
            to_send: HashMap::new(),
            promised: HashSet::new(),
            accepted: HashSet::new(),
            aval: 0,
            aid: 0,
            status: Status::Active,
            prom_state: false
        }
    }
}

impl fmt::Display for Proposer{

    /// Function to format printing of Proposer object
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Proposer => num_acceptors: {}, id: {}, val: {}, status: {}", self.num_acceptors, self.id, self.val, self.status)
    }

}