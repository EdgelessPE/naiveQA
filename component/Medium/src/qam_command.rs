use crate::qam_struct::{CommandPayload,CommandOption};

pub fn process_command(payload:CommandPayload)->Result<String,String>{
    println!("{:?}",payload);
    return Ok("".to_owned())
}