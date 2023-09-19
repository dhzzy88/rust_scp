use bincode::{config, Decode, Encode};
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Sendmessge{
    pub filename: String,
    pub total_size: u64,
    pub end:bool,
    pub seq:u64,
    pub size:u64,
}

impl Sendmessge {
        pub fn new() -> Sendmessge{
            Sendmessge{
                filename:String::from("test.bin"),
                total_size:10,
                end:true,
                seq:1,
                size:10,
            }
        }
        pub fn encode(&self) -> Vec<u8> {
            let config = config::standard();
            let encoded: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();
            return encoded;
        }
}

impl Encodestruct for Sendmessge{
    fn encode(&self) -> Vec<u8> {
        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();
        return encoded;
    }
}

impl Decodestruct<Sendmessge> for Sendmessge {
    fn decode(&mut self,data:&Vec<u8>) {
        let config = config::standard();
        let (decoded,len): (Sendmessge,usize) = bincode::decode_from_slice(&data[..], config).unwrap();
        self.seq = decoded.seq;
        self.size = decoded.size;
        self.end = decoded.end;
        self.total_size = decoded.total_size;
        self.filename = decoded.filename;
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Recvmessge {
    pub filename: String,
    pub total_size: u64,
    pub end:bool,
    pub seq:u32,
    pub size:u64,
}

impl Recvmessge {
    pub fn new() -> Recvmessge{
        Recvmessge{
            filename:String::from("test.bin"),
            total_size:10,
            end:true,
            seq:0,
            size:10,
        }
    }
}

impl Encodestruct for Recvmessge{
    fn encode(&self) -> Vec<u8> {
        let config = config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();
        return encoded;
    }
}

impl Decodestruct<Recvmessge> for Recvmessge{
    fn decode(&mut self,data:&Vec<u8>) {
        let config = config::standard();
        let (decoded, len): (Recvmessge, usize) = bincode::decode_from_slice(&data[..], config).unwrap();
        self.end = decoded.end;
        self.seq = decoded.seq;
        self.filename = decoded.filename;
        self.size = decoded.size;
        self.total_size = decoded.total_size;
    }
}

pub trait Encodestruct{
    fn encode(&self) -> Vec<u8>;
}

pub trait Decodestruct<T>{
    fn decode(&mut self,data:&Vec<u8>);
}