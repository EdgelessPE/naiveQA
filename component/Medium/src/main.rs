mod qam_struct;
mod qam_command;

extern crate websocket;
extern crate argparse;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use argparse::{ArgumentParser, Store};

fn main() {
	//配置变量/常量
	const VERSION: &'static str=env!("CARGO_PKG_VERSION");
	let mut port="14514".to_string();

	//解析命令行参数
	{
		let mut ap=ArgumentParser::new();
		ap.refer(&mut port).add_option(&["-p"],Store,"Port Medium Guest listening to");
		ap.parse_args_or_exit();
	}

	//输出初始化信息
	println!("naiveQA Medium Guest @ver{}",VERSION);
	println!("Listening to {}",port);

	//尝试监听端口
	let server_result = Server::bind("127.0.0.1:".to_owned()+&port);
	if let Err(e)=server_result{
		println!("Failed to listen port {},try -p argument to change the default port:{}",&port,e.to_string());
		std::process::exit(1);
	}
	let server=server_result.unwrap();

	//遍历连接请求
	for request in server.filter_map(Result::ok) {
		//为每个请求新建进程处理
		thread::spawn(|| {
			//接受请求连接
			let mut client = request.accept().unwrap();
			let ip = client.peer_addr().unwrap();
			println!("Connection from {}", ip);

			//发送medium握手信息
			let message = OwnedMessage::Text(("Medium Guest Version=".to_owned()+&VERSION).to_string());
			client.send_message(&message).unwrap();

			//监听消息
			let (mut receiver, mut sender) = client.split().unwrap();
			for message in receiver.incoming_messages() {
				let message = message.unwrap();
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Text(text)=>{
						println!("{}",&text);
						//解析Json
						let json_r:Result<qam_struct::WebsocketContainer, _> =serde_json::from_str(&text);
						if let Err(_)= json_r {
							println!("Can't parse into json:{}",&text);
							continue;
						}
						let container=json_r.unwrap();
						//println!("{:?}",container);

						//根据任务类型分流处理
						match &container.task[..] {
							//测试用例：{"id":"1","task":"Command","direction":0,"payload":"{\"command\":\"ls\",\"option\":{\"pwd\":\"\",\"env\":\"\",\"encoding\":\"\",\"timeout\":6000,\"shell\":\"cmd\"}}"}
							"Command" => {
								//解析Command payload
								let cmd_pld_str=&container.payload[..];
								let cmd_pld_res:Result<qam_struct::CommandPayload,_>=serde_json::from_str(&cmd_pld_str);
								if let Err(e)=cmd_pld_res{
									println!("Can't parse into CommandPayload:{},for:{}",&cmd_pld_str,e.to_string());
									continue;
								}
								let cmd_pld=cmd_pld_res.unwrap();
								//传入处理函数
								let tmp= qam_command::process_command(cmd_pld);
							},
							"File"=>{
								println!("Exec file")
							},
							_ => {
								println!("Invalid command:{}",&container.task)
							}
						}
					}
					_ => {
						println!("Got illegal message,ignore");
					}
				}
			}
		});
	}
}