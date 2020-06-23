use std::fmt;
use crate::messages::Message;
use crate::messages::Status;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Acceptor{
    max_known_id: u32,
    id: u32,
    val: u32,
    accepted_id: u32,
    pub status: Status,
    to_send: HashMap<u32, Message>
}

impl Acceptor{

    /// Getter for max_known_id
    ///
    /// # Returns
    ///
    /// u32 - Largest id seen from received messages.
    ///
    pub fn max_known_id(&self) -> u32{
        self.max_known_id
    }

    /// Getter for id
    ///
    /// # Returns
    ///
    /// u32 - Id of the Acceptor.
    ///
    pub fn id(&self) -> u32{
        self.id
    }

    /// Getter for val
    ///
    /// # Returns
    ///
    /// u32 - Val from the largest seen id.
    ///
    pub fn val(&self) -> u32{
        self.val
    }

    /// Setter for max_known_id
    ///
    /// # Arguments
    /// 
    /// * `id` - Largest Proposer id seen.
    ///
    pub fn set_max_known_id(&mut self, id: u32){
        self.max_known_id = id;
    }

    /// Setter for id
    ///
    /// # Arguments
    /// 
    /// * `id` - Id of the Acceptor.
    ///
    pub fn set_id(&mut self, id: u32){
        self.id = id;
    }

    /// Setter for val
    ///
    /// # Arguments
    /// 
    /// * `val` - Value received from Proposer.
    ///
    pub fn set_val(&mut self, val: u32){
        self.val = val;
    }

    /// Setter for status
    ///
    /// # Arguments
    /// 
    /// * `status` - Status of the Acceptor.
    ///
    pub fn set_status(&mut self, status: Status){
        self.status = status;
    }

    /// Function to check a received message and choose what to do with it. This could 
    /// be liken to a traditonal state transition function where it decides what Acceptor 
    /// should do depending on the message that is being read.
    ///
    /// # Arguments
    /// 
    /// * `message` - Message received from Acceptor
    ///
    /// # Returns
    ///
    /// Message to sent to Acceptor
    ///
    fn check_message(&mut self, message: Message) -> Message{
        match message {
            Message::Prepare(i, _) => {
                if i <= self.max_known_id {
                    Message::Fail(i, self.id)
                }
                else {
                    self.max_known_id = i;
                    if self.status == Status::Accepted{
                        Message::AcceptedPromise(i, self.accepted_id, self.val, self.id)
                    }
                    else {
                        self.status = Status::Promised;
                        Message::Promise(i, self.id)
                    }
                }
            },
            Message::Propose(i,v, _) => {
                if self.max_known_id == i {
                    self.val = v;
                    self.status = Status::Accepted;
                    self.accepted_id = i;
                    Message::Accepted(i, v, self.id)
                }
                else{
                    Message::Fail(i, self.id)
                }
            },
            _ => Message::Error
        }
    }

    /// Function to check the buffer of the Acceptor. It will check for any messages that are
    /// intended for the Acceptor and will determine what the reply should be, if one is needed 
    /// and it will transition the proposer between states.
    ///
    /// # Arguments
    /// 
    /// * `buffer` - Shared buffer where messages are shared between nodes in the network
    ///
    pub fn check_buffer(&mut self, buffer: &Arc<HashMap<u32, Mutex<Vec<Message>>>>) {
        if (*buffer).contains_key(&self.id) && self.status != Status::Failed {
            let mut bucket = (*buffer).get(&self.id).unwrap().lock().unwrap();
            while bucket.len() > 0 {
                let message = self.check_message(bucket.pop().unwrap());
                match message {
                    Message::Promise(id, _) => self.to_send.insert(id, message),
                    Message::Fail(id, _) => self.to_send.insert(id, message),
                    Message::Accepted(id, _, _) => self.to_send.insert(id, message),
                    _ => panic!("Wrong message created"),
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
                    println!("Sending ({}) to Prop {} from Acc {}", &v, &k, &self.id);
                    let mut bucket = (*buffer).get(&k).unwrap().lock().unwrap();
                    bucket.push(v);
                }
            }
        }
    }
}

impl Default for Acceptor{

    /// Default function for Acceptor which will generate a basic 
    /// Acceptor object and return it
    ///
    /// # Returns
    ///
    /// Acceptor with all variables set to default values.
    ///
    fn default() -> Acceptor{
        Acceptor{
            max_known_id: 0,
            id: 0,
            val: 0,
            accepted_id: 0,
            status: Status::Active,
            to_send: HashMap::new(),
        }
    }
}

impl fmt::Display for Acceptor{

    /// Function to format printing of Acceptor object
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Acceptor => max_known_id: {}, id: {}, val: {}, status: {}", self.max_known_id, self.id, self.val, self.status)
    }

}