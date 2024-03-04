use chrono::{DateTime, Local};



pub struct Conversation {
    messages: Vec<(String, DateTime<Local>)>,

}


impl Conversation {

    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new(),
        }       
    }

    fn get_messages(&self) -> &Vec<(String, DateTime<Local>)> {
        &self.messages
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push((message, Local::now()));
    }

}

#[cfg(test)]
mod tests {

    use super::Conversation;

    #[test]
    fn test_should_get_messages_return_empty_vec() {
        let conversation = Conversation::new();
        let messages = conversation.get_messages();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_should_add_message_add_a_message() {
        let mut conversation = Conversation::new();
        conversation.add_message(String::from("some message"));
        let messages = conversation.get_messages();
        assert!(!messages.is_empty());
        assert_eq!(messages[0].0,"some message");
    }



}