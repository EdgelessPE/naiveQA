extern crate websocket;
extern crate argparse;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use argparse::{ArgumentParser, Store};

fn main() {
	//配置变量
	let mut port="14514".to_string();
	//解析参数
	{
		let mut ap=ArgumentParser::new();
		ap.refer(&mut port).add_option(&["-p"],Store,"Port Medium Guest listening to");
		ap.parse_args_or_exit();
	}
	//输出初始化信息
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
			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

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
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
}