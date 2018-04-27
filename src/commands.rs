use telegram_bot::Message;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct BotError(Option<String>);

impl error::Error for BotError {
    fn description(&self) -> &str {
        match self.0 {
            Some(ref m) => m,
            None => "something went wrong",
        }
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use std::error::Error;

        f.write_str(self.description())
    }
}

trait Command {
    fn process_command(message: &Message) -> Result<(), BotError>;
}

struct Spam;

impl Command for Spam {
    fn process_command(message: &Message) -> Result<(), BotError> {
        if let Some(ref reply) = message.reply_to_message {
        }

        Ok(())
    }
}
