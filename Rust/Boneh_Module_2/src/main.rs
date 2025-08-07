use std::{fmt::Write, num::ParseIntError};
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray, generic_array::typenum::U16
};


pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn decode_cbc(key : &String, dataIn : String) -> String
{
     let mut keyVec: Vec<u8> = decode_hex(&key).unwrap();
     let mut dataVec: Vec<u8> = decode_hex(&dataIn).unwrap();

     let (iv, data) = dataVec.split_at_mut(16);

     let keyGa: GenericArray<u8, U16> = GenericArray::clone_from_slice(keyVec.as_slice());

    let cipher = Aes128::new(&keyGa);

    let mut block_vec : Vec< GenericArray<u8, U16>  > = Vec::new();
    let mut block_idx = 0;
    let mut block: GenericArray<u8, U16>;
    for i in (0..data.len()).step_by(16)
    {
        let mut end = 16;
        if data.len() < i + 16
        {
            end = data.len() - i;
            println!("{}", end);
            let mut final_block: Vec<u8> = vec![0; 16];
            final_block[0..end].copy_from_slice(&data[i..data.len()]);
            println!("{:?}", final_block);

            block = GenericArray::clone_from_slice(final_block.as_slice());
        }
        else
        {
         block = GenericArray::clone_from_slice(&data[i..i+16]);
        }

        cipher.decrypt_block(&mut block);

        let mut xor_val: GenericArray<u8, U16>;
        if block_idx == 0 
        {
            xor_val = GenericArray::clone_from_slice(iv);
        }
        else
        {
            //cbc
            xor_val = GenericArray::clone_from_slice(&data[i-16..i]);

        }

        //println!("Block Before: {:?}", block);
       // println!("XOR Val: {:?}", xor_val);
        for j in 0..16
        {
            block[j] = block[j] ^ xor_val[j];
        }

        block_vec.push(block);
        block_idx+=1;

        //println!("DEC: {:?}", block);
        
    }

    //let mut decoded_vec: Vec<u8> = Vec::new();
    for i in block_vec
    {
        for j in 0..16
        {
            print!("{}",i[j] as char);
        }
    }
    print!("\n\r");

    //too lazy to convert to string
    "hello".to_owned()
}

fn decode_ctr(key : &String, dataIn : String) -> String
{
     let mut keyVec: Vec<u8> = decode_hex(&key).unwrap();
     let mut dataVec: Vec<u8> = decode_hex(&dataIn).unwrap();

     let (iv, data) = dataVec.split_at_mut(16);

     let keyGa: GenericArray<u8, U16> = GenericArray::clone_from_slice(keyVec.as_slice());

    let cipher = Aes128::new(&keyGa);

    let mut block_vec : Vec< GenericArray<u8, U16>  > = Vec::new();
    let mut block_idx = 0;
    let mut block: GenericArray<u8, U16>;
    for i in (0..data.len()).step_by(16)
    {
        let mut end = 16;
        if data.len() < i + 16
        {
            end = data.len() - i;
            //println!("{}", end);
            let mut final_block: Vec<u8> = vec![0; 16];
            final_block[0..end].copy_from_slice(&data[i..data.len()]);
            //println!("{:?}", final_block);

            block = GenericArray::clone_from_slice(final_block.as_slice());
        }
        else
        {
         block = GenericArray::clone_from_slice(&data[i..i+16]);
        }

       

        let mut xor_val: GenericArray<u8, U16>;
         //ctr
        xor_val = GenericArray::clone_from_slice(iv);
        //hack because i am lazy 
        xor_val[15] += block_idx;//obviously this will overflow at some point, I dont want to do bignums with these jank generic arrays.
        
        cipher.encrypt_block(&mut xor_val);
        //println!("Block Before: {:?}", block);
       // println!("XOR Val: {:?}", xor_val);
        for j in 0..16
        {
            block[j] = block[j] ^ xor_val[j];
        }

        

        block_vec.push(block);
        block_idx+=1;

        //println!("DEC: {:?}", block);
        
    }

    //let mut decoded_vec: Vec<u8> = Vec::new();
    for i in block_vec
    {
        for j in 0..16
        {
            print!("{}",i[j] as char);
        }
    }
    print!("\n\r");

    //too lazy to convert to string
    "hello".to_owned()
}



fn main() {
    let cypherText = String::from("4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81");
    let cbcKey = String::from("140b41b22a29beb4061bda66b6747e14");
    decode_cbc(&cbcKey,cypherText);
    let cypherText = String::from("5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253");
    decode_cbc(&cbcKey,cypherText);

    //ctr
    let ctrKey = String::from("36f18357be4dbd77f050515c73fcf9f2");
    let cypherText = String::from("69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329");
    decode_ctr(&ctrKey,cypherText);

    let cypherText = String::from("770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451");
    decode_ctr(&ctrKey,cypherText);

    
}

