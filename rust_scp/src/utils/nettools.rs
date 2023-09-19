use pnet::datalink;
use std::net::{IpAddr,Ipv4Addr};
pub fn get_eth0_ipv4_addr() -> IpAddr {
    for interface in datalink::interfaces(){
        if interface.name == String::from("eth0") || interface.name == "eth01".to_string() || interface.name == String::from("en0")  {
            // println!("Interface: {:?}", interface.name);
            for ip in interface.ips {
                // println!("IP: {:?}", ip.ip);
                if ip.is_ipv4() {
                    return ip.ip();
                }
            }
        }
    }
    return IpAddr::V4(Ipv4Addr::new(127,0,0,1));
}