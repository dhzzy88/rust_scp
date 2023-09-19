use std::path::{Path};
use tokio::fs;
use std::net::{SocketAddr,Ipv4Addr,IpAddr};
use tokio::net::{TcpListener,TcpStream};
use std::io::{Read};
use std::str::FromStr;
use crate::utils::message;
use crate::utils::message::{Decodestruct};
use tokio::io::{AsyncReadExt};
use crate::utils::nettools;

#[derive(Debug)]
pub struct Server<'a> {
    path:&'a str,
    ip:IpAddr,
    filecount:usize,
    port:u16,
}
impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            path: "default.bin",
            ip: nettools::get_eth0_ipv4_addr(),
            filecount: 0,
            port: 3000
        }
    }
    pub fn set_attr(&mut self,path:&'a str,ip:&'a str,port:u16){
        self.path = path;
        self.port = port;
        if ip != "127.0.0.1" {
            self.ip = IpAddr::V4(Ipv4Addr::from_str(ip).unwrap());
        }
        println!("ip:{},port:{}",self.ip,self.port);
    }
    pub async fn process(&mut self,stream:&mut TcpStream) {
        let mut buf = vec![0; 1024];
        let mut sendmesg = message::Sendmessge::new();
        match stream.read(&mut buf).await {
            Ok(n) => {
                sendmesg.decode(&buf);
            },
            Ok(0) => {
                {};
            },
            Err(_) => {
                println!("read error !");
            }
        }
        let path_info = Path::new(&(sendmesg.filename)).file_name().unwrap().to_str().unwrap();
        let mut file_stream = fs::File::create(path_info).await.unwrap();
        let ret= tokio::io::copy(stream,&mut file_stream).await.unwrap();
        let passed = self.check_file(sendmesg.total_size.try_into().unwrap(),ret);
        println!("{:?}",sendmesg);
        println!("recived byte {:?},check size {},ip {}",ret,passed,&stream.peer_addr().unwrap().ip());
    }

    fn check_file(&self,total_size:u64,read_size:u64) -> bool{
         return total_size == read_size;
    }
   async fn listen(&mut self,listener:&TcpListener) {
       loop {
           let (mut stream,addr) = listener.accept().await.unwrap();
           self.process(&mut stream).await;
       }
    }

    pub  async fn server_main( &mut self ) {
        println ! ("server");
        let socketaddr = SocketAddr::new(self.ip,self.port);
        let listener = TcpListener::bind(&socketaddr).await.unwrap();
        self.listen(&listener).await;
    }
}
