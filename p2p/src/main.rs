use std::net::{TcpStream};
use std::io::{Read, Write};
use hex;

fn main() {
    match TcpStream::connect("127.0.0.1:9876") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 9876");

            let msg = hex::decode("0900000006421b0000a51b0000").unwrap();

            stream.write(msg.as_slice()).unwrap();
            println!("Sent, awaiting reply...");

//            let mut buffer = Vec::new();
//            let length = stream.re(&mut buffer).unwrap();
//            let text = hex::encode(buffer);
//            println!("Response: {}", text);

            let mut data = [0u8; 18900];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = hex::encode(&data.as_ref());
                    println!("Response: {}", text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

//b900000007dded404a0000000000ea3055000000001b41d39f263026aa8916529450c964a8724a2d71498dbcefead211a24f720000000000000000000000000000000000000000000000000000000000000000bf17e8f5e8024c2f017f7861004750287b861c08ddb74b15c848ebf3bde11afd000000000000001f6db047c02fb436bd3c6d04593b5d3254be0f72a6c747453ef66d4d4c7b7987a128705a976b8f653997849b6c17191866be8d2f384ea01cac75eb1fecf67c7e910000
//          397c905d0000000000ea305500000a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0000000000000001010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010000

//b900000007deed404a0000000000ea3055000000001b42e1865ae6d0ba48f2301e29954e34f503f51f205985904e8d3790efae0000000000000000000000000000000000000000000000000000000000000000cefdbf029730dfa4bb30e8d711c43bbebb63c733d37fe0bcaa6eac4882c5954a00000000000000206f04507fdfe206e480579f97e23ff0e9a3f8056e3ad001ff03ec546c0f04a84554c23ae30a2b286b89bb036bb41497c018e4b76addb9427617a30f694f85aa6a0000
//b900000007dfed404a0000000000ea3055000000001b4375152440f84249c03a5f8c78cd23e6ab734e2487396685acf9fc80f70000000000000000000000000000000000000000000000000000000000000000c873cb064fe4e18054b8f88565b5bac029dfa77d6ee3c4a6acb79bab704c262d000000000000001f096af6ed423adcf732cdbc857d992d6ba7068e0e8c95559637e672eb7edb40d251452046da9bdb8cae6de91095f8a3d2f454ace6b2b2749f6f9dcdaa927ebbb10000
//b900000007e0ed404a0000000000ea3055000000001b447fda7e71444537b5970b0ceb29158fe4ee3135c922e18b4e91158a3f00000000000000000000000000000000000000000000000000000000000000004dd2197d80f23ecb55dc3e1b06f08ce5fde49f383344e114612b8c1f0734e95400000000000000201cc50f1ff9db6b8ad16ea76ae10f7020ea1c66830f45bb28af703235fb366cfa7c2d598ead2c044c2bdda651107444d422e39b7fd2447210dd86367cb7e15ce70000
//b900000007e1ed404a0000000000ea3055000000001b458f9bf924ebe7803351343d6b5129fca91226c774d9d42dc93a4656d00000000000000000000000000000000000000000000000000000000000000000f3286bc60713d95bf6fc7342df4beca7c0bc52ba72dcc41ef7455d54b6d8d645000000000000001f356b9da5303d2c11823d938fd17ded19c2bb1ab574692e2e30cc6a34a20f4c993c7ce6f0ea70a80d39d35fbf1aca95c0e7840979702b1f04987b1d95ecc821a10000
//b900000007e2ed404a0000000000ea3055000000001b462d5897cb83e1f8d1abbc1de6c8ec246a1f57b7f8cd8106af136bc32900000000000000000000000000000000000000000000000000000000
