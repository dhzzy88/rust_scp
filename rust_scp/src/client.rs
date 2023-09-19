use std::path::{Path};
use tokio::fs;
use std::io::{Read,Write};
use tokio::net::{TcpStream};
use tokio::io::{self,split,copy};
use tokio::spawn;
use std::net::{Ipv4Addr};
use std::str;
use std::str::FromStr;
use super::utils::message;
use tokio::io::AsyncWriteExt;


#[derive(Debug)]
pub struct Client<'a> {
    path:&'a str,
    ip:Ipv4Addr,
    filecount:usize,
    port:u16
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Client {
            path: "./",
            ip: Ipv4Addr::new(127,0,0,1),
            filecount: 0,
            port:3000,
        }
    }
    pub fn set_attr(&mut self,path:&'a str,ip:&'a str,port:u16){
        self.path = path;
        self.ip = Ipv4Addr::from_str(ip).unwrap();
        self.port = port;
    }

    pub async fn client_main( &self ) {
        println ! ("client");
        let start = std::time::Instant::now();
        let msg = message::Sendmessge::new();
        let socketaddr = self.ip.to_string() + ":" + &self.port.to_string();
        let socket = TcpStream::connect(socketaddr).await.unwrap();
        let (rd, mut wr) = io::split(socket);
        let mut file_stream = fs::File::open(self.path).await.unwrap();
        let metadata = file_stream.metadata().await.unwrap();
        let mut sendmsg = message::Sendmessge::new();
        sendmsg.filename = String::from(self.path);
        sendmsg.total_size = metadata.len() as u64;
        sendmsg.size = sendmsg.total_size;
        let buf = sendmsg.encode();
        wr.write_all(&buf).await;
        wr.flush().await.unwrap();
        let ret = tokio::io::copy(&mut file_stream,&mut wr).await.unwrap();
        let end = std::time::Instant::now();
        let speed = speed_computer(start,end,ret);
        println!("send byte :{},speed :{:?}",ret,speed);
    }
}
const TB:f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
const GB:f64 = 1024.0 * 1024.0 * 1024.0 ;
const MB:f64 = 1024.0 * 1024.0 ;
const KB:f64 = 1024.0 ;
fn speed_computer(start:std::time::Instant,end:std::time::Instant,total_size:u64)-> String {
        let elapsed = end.duration_since(start);
        let speed_bytes = total_size as f64 / elapsed.as_secs_f64();
        let mut speed = String::new();
        match speed_bytes {
            GB..=TB => {
                speed =  (speed_bytes/GB).to_string() + " GB/s";
            },
            MB..=GB => {
                speed =  (speed_bytes/MB).to_string() + " MB/s"
            },
            KB..=MB => {
                speed = (speed_bytes/KB).to_string() + " KB/s"
            },
            _ => {speed = speed_bytes.to_string() + " Bytes/s"}
        };
        return speed;
}
