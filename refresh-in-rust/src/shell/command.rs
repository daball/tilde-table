use crate::shell::param::FormalParam;

pub struct Command {
    aliases: Vec<String>,
    category: String,
    short_description: String,
    long_description: String,
    params: Vec<FormalParam>,
}
