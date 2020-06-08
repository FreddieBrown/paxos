use std::fmt;

pub enum Status{
    Active,
    Proposed,
    Accepted,
    Failed
}

impl fmt::Display for Status{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &self {
            Status::Active => "Active",
            Status::Proposed => "Proposed",
            Status::Accepted => "Accepted",
            Status::Failed => "Failed"
        };
        write!(f, "{}", printable)
    }
}

pub struct Acceptor{
    max_known_id: u32,
    id: u32,
    val: u32,
    status: Status,
}

impl Acceptor{

    pub fn max_known_id(&self) -> u32{
        self.max_known_id
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

    pub fn set_max_known_id(&mut self, id: u32){
        self.max_known_id = id;
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
}

impl Default for Acceptor{
    fn default() -> Acceptor{
        Acceptor{
            max_known_id: 0,
            id: 0,
            val: 0,
            status: Status::Active,
        }
    }
}

impl fmt::Display for Acceptor{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Acceptor => max_known_id: {}, id: {}, val: {}, status: {}", self.max_known_id, self.id, self.val, self.status)
    }

}