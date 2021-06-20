pub mod qam_struct {
    pub struct WebsocketInterface {
        id:String,
        task:String,
        direction:u8,
        payload:String
    }
    pub struct FilePayload{
        src:String,
        dst:String,
        current_page:u64,
        total_page:u64,
        page:buf
    }
    pub struct CommandPayload{
        command:String,
        option:CommandOption
    }
    pub struct CommandOption{
        pwd:String,
        env:String,
        encoding:String,
        shell:String,
        timeout:u64
    }
}