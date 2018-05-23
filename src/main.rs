#![recursion_limit = "1024"]

extern crate proxyconf;

fn main() {
    let conf = proxyconf::registry::read().unwrap();
    println!("conf = {:?}", conf);
    let mut bytes = Vec::new();
    proxyconf::serialization::serialize(&conf, &mut bytes).unwrap();
    println!("bytes = {:?}", bytes);
    let conf2 = proxyconf::serialization::deserialize(&bytes[..]).unwrap();
    println!("conf2X = {:?}", conf2);
}
