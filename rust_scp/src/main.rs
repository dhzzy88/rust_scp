mod client;
mod server;
mod utils;
use clap::Parser;
use tokio;
#[derive(Parser,Debug)]
#[command(author,version,about,long_about = None)]
struct Args{
    #[arg(short,long)]
    role:String,
    #[arg(short,long,default_value="default.bin")]
    file:String,
    #[arg(short,long,default_value="127.0.0.1")]
    ip:String,
    #[arg(short,long,default_value_t=3000)]
    port:u16,

}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.role == "server" {
        true => {
            let mut s = server::Server::new();
            s.set_attr(&(args.file),&args.ip,args.port);
            s.server_main().await;

        },
        _ => {
            let mut c = client::Client::new();
            c.set_attr(&(args.file),&args.ip,args.port);
            c.client_main().await;
        }
    }
}
