use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WebsocketContainer {
    pub id:String,
    pub task:String,
    pub direction:u8,
    pub payload:String
}

// #[derive(Deserialize, Debug)]
// pub struct FilePayload{
//     src:String,
//     dst:String,
//     current_page:u64,
//     total_page:u64,
//     payload:[u8]
// }

#[derive(Deserialize, Debug)]
pub struct CommandPayload{
    command:String,
    option:CommandOption
}

#[derive(Deserialize, Debug)]
pub struct CommandOption{
    pwd:String,
    env:String,
    encoding:String,
    shell:String,
    timeout:u64
}