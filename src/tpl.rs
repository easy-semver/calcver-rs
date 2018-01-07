pub fn is_template_valid(template: String) -> bool {
    true
}

pub fn is_message_valid(msg: String, tpl: String) -> bool {
    true
}

pub fn get_message(msg: String) -> super::Message {
    let retval = super::Message {
        message_type: String::from("tes"),
        scope: String::from("test"),
        short_description: String::from("test"),
        description: String::from("test"),
        breaking_change: false,
        raw: String::from("")
    };
    retval
}

pub fn create_message(msg: super::Message) -> String {
    "".to_string()
}