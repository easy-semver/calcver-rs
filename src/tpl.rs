

        pub struct Message {
            message_type: String,
            scope: String,
            short_description: String,
            description: String,
            breaking_change: bool,
            raw: String,
        }

        pub fn is_template_valid(template: String) -> bool {
            true
        }

        pub fn is_message_valid(msg: String, tpl: String) -> bool {
            true
        }

        pub fn get_message(msg: String) -> Message {
            let mut retval = Message {
                message_type: String::from("tes"),
                scope: String::from("test"),
                short_description: String::from("test"),
                description: String::from("test"),
                breaking_change: false,
                raw: String::from("")
            };
            retval
        }
 