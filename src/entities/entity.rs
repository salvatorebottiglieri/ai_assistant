use chrono::{DateTime, Local};



pub struct Conversation {
    messages: (Vec<String>,DateTime<Local>),

}


impl Conversation {

    pub fn new() -> Conversation {
        Conversation {
            messages: (Vec::new(),Local::now())
        }       
    }

}