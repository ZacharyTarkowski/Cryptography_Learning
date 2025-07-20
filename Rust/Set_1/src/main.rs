
use std::{fmt::Write, num::ParseIntError};

use std::collections::HashMap;
use std::sync::OnceLock;



use std::io;
use std::io::prelude::*;
use std::fs::File;
use base64::prelude::*;


const BASE64_MASK_1  : u8 = 0xFC;
const BASE64_MASK_2_1: u8 = 0x03;
const BASE64_MASK_2_2: u8 = 0xF0;
const BASE64_MASK_3_1: u8 = 0x0F;
const BASE64_MASK_3_2: u8 = 0xC0;
const BASE64_MASK_4  : u8 = 0x3F;

static TABLE_BASE64: &'static [u8] = & [
    b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z',b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'+',b'/'
    ];


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


fn to_base64(s: &str) -> Vec <u8>
{
    let hex_string = decode_hex(s).unwrap();

    assert!(hex_string.len() % 0x3 == 0);

    let mut result = Vec::new();

    let mut i = 0;
    let mut out_idx = 0;

    while i < hex_string.len()
    {
        let state = out_idx&0x3;
        let val = hex_string[i];
        match state
        {
            0=> 
            result.push( (val & BASE64_MASK_1) >> 2 ) ,
            1=> 
            result.push( ((val & BASE64_MASK_2_1) << 4)  | ((hex_string[i+1] & BASE64_MASK_2_2) >> 4) ),
            2=> 
            result.push( ((val & BASE64_MASK_3_1) << 2)  | ((hex_string[i+1] & BASE64_MASK_3_2) >> 6) ),
            3=>
            result.push( val & BASE64_MASK_4 ),
            _=> (),
        }
        //println!("{}",state);

        i = if out_idx & 0x03 != 0 {i+1} else { i };
        out_idx += 1;
    }

    //println!("{:?}", result);
    return result;
}

fn print_base64(v : Vec<u8>)
{
    for i in v
    {
        print!("{}", TABLE_BASE64[i as usize] as char);
    }
    print!("/n/r");
}

fn challenge_1()
{
    let base_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let s = base_string.as_str();
    let result = to_base64(s);
    print_base64(result);
}

fn arr_xor( in_1 : &[u8], in_2 : &[u8], out : &mut [u8] )
{
    assert!(in_1.len() == in_2.len() && in_1.len() == out.len());

    for i in 0..in_1.len()
    {
        out[i] = in_1[i] ^ in_2[i];
    }
}

fn hex_str_xor( in_1 : &str, in_2 : &str ) -> String
{
    let in_1_vec = decode_hex(in_1).unwrap();
    let in_2_vec = decode_hex(in_2).unwrap();

    let mut out_vec : Vec<u8> =  vec![0; in_1_vec.len()];

    arr_xor(in_1_vec.as_slice(), in_2_vec.as_slice(), out_vec.as_mut_slice());

    return encode_hex(out_vec.as_slice());
    
}


fn challenge_2()
{
    let in_1_str = "1c0111001f010100061a024b53535009181c";
    let in_2_str = "686974207468652062756c6c277320657965";

    println!("{}", hex_str_xor(in_1_str,in_2_str));
}

//cornell math english letter
fn letter_frequency_map() -> &'static HashMap<u8 , u32> {
    static HASHMAP: OnceLock<HashMap<u8, u32>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert( b'E' , 1202 );
        m.insert( b'T' , 910  );
        m.insert( b'A' , 812  );
        m.insert( b'O' , 768  );
        m.insert( b'I' , 731  );
        m.insert( b'N' , 695  );
        m.insert( b'S' , 628  );
        m.insert( b'R' , 602  );
        m.insert( b'H' , 592  );
        m.insert( b'D' , 432  );
        m.insert( b'L' , 398  );
        m.insert( b'U' , 288  );
        m.insert( b'C' , 271  );
        m.insert( b'M' , 261  );
        m.insert( b'F' , 230  );
        m.insert( b'Y' , 211  );
        m.insert( b'W' , 209  );
        m.insert( b'G' , 203  );
        m.insert( b'P' , 182  );
        m.insert( b'B' , 149  );
        m.insert( b'V' , 111  );
        m.insert( b'K' , 069  );
        m.insert( b'X' , 017  );
        m.insert( b'Q' , 011  );
        m.insert( b'J' , 010  );
        m.insert( b'Z' , 007  );
        m.insert( b'e' , 1202 );
        m.insert( b't' , 910  );
        m.insert( b'a' , 812  );
        m.insert( b'o' , 768  );
        m.insert( b'i' , 731  );
        m.insert( b'n' , 695  );
        m.insert( b's' , 628  );
        m.insert( b'r' , 602  );
        m.insert( b'h' , 592  );
        m.insert( b'd' , 432  );
        m.insert( b'l' , 398  );
        m.insert( b'u' , 288  );
        m.insert( b'c' , 271  );
        m.insert( b'm' , 261  );
        m.insert( b'f' , 230  );
        m.insert( b'y' , 211  );
        m.insert( b'w' , 209  );
        m.insert( b'g' , 203  );
        m.insert( b'p' , 182  );
        m.insert( b'b' , 149  );
        m.insert( b'v' , 111  );
        m.insert( b'k' , 069  );
        m.insert( b'x' , 017  );
        m.insert( b'q' , 011  );
        m.insert( b'j' , 010  );
        m.insert( b'z' , 007  );
        
        let mut avg : u32 = 0;
        for (_k, v) in m.iter()
        {
            avg += *v;
        }

        avg /= m.len() as u32;

        m.insert(0, avg);

        

        return m;
    })
}

fn score_english(in_vec : &[u8]) -> u32
{
    let mut avg: u32 = 0;
    if in_vec.len() > 0
    {
    
        for c in in_vec
        {
            if (*c).is_ascii_alphabetic()
            {
                //println!("{}", letter_frequency_map().get(c).unwrap());

            avg += letter_frequency_map().get(c).unwrap();
            }
        }

    avg /= in_vec.len() as u32;
    }   
    
    return avg;
}

fn str_xor( in_1 : &str, in_2 : &str ) -> String
{
    
    let mut out_vec : Vec<u8> =  vec![0; in_1.len()];

    arr_xor(in_1.as_bytes(), in_2.as_bytes(), out_vec.as_mut_slice());

    return String::from_utf8(out_vec).unwrap();
    
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn check_single_letter_xor(challenge_string : &str)
{
    let challenge_vec = decode_hex(challenge_string.trim()).unwrap();

    

    for i in 0..=127
    {
        let single_char_string = String::from_utf8(vec![i; challenge_vec.len()]).unwrap(); 

        let mut out_vec : Vec<u8> =  vec![0; challenge_vec.len()];

        arr_xor(challenge_vec.as_slice(), single_char_string.as_bytes(), out_vec.as_mut_slice() ) ;
        let out_score = score_english(out_vec.as_slice() );

        let out_str_wrapped = String::from_utf8(out_vec);
        if out_score >= *letter_frequency_map().get(&0).unwrap() && out_str_wrapped.is_ok()
        {
            println!("{} {} {}", out_str_wrapped.unwrap(), out_score, i as char);
        }
     }
}

fn check_single_letter_xor_vec(challenge_vec : Vec<u8>) -> char
{
    let mut highest_score = 0;
    let mut highest_letter: char = '0';
    for i in 0..=127
    {
        let single_char_string = String::from_utf8(vec![i; challenge_vec.len()]).unwrap(); 

        let mut out_vec : Vec<u8> =  vec![0; challenge_vec.len()];

        arr_xor(challenge_vec.as_slice(), single_char_string.as_bytes(), out_vec.as_mut_slice() ) ;
        let out_score = score_english(out_vec.as_slice() );

        let out_str_wrapped = String::from_utf8(out_vec);
        if out_score >= *letter_frequency_map().get(&0).unwrap() && out_str_wrapped.is_ok()
        {
            println!("{} {} {}", out_str_wrapped.unwrap(), out_score, i as char);
        }

        if(out_score > highest_score)
        {
            highest_score = out_score;
            highest_letter = i as char;
        }
     }

     return highest_letter;
}

fn challenge_3()
{
    println!("Average letter frequency is {}", letter_frequency_map().get(&0).unwrap());
    //let challenge_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //check_single_letter_xor(challenge_string);

    let challenge_string = "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f";
    check_single_letter_xor(challenge_string);
}

fn challenge_4()
{
    let file_path = "src/challenge_4_data.txt";
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        check_single_letter_xor(line.unwrap().as_str().trim());
    }

    
}

fn repeating_key_xor(key : &[u8], data : &[u8], out : &mut [u8])
{
    assert!(data.len() <= out.len() && key.len() <= data.len());

    let mut key_iter = 0;
    for i in 0..data.len()
    {
        out[i] = data[i] ^ key[key_iter];

        key_iter += 1;
        if key_iter >= key.len()
        {
            key_iter = 0;
        }
    }
}

fn challenge_5()
{
    let key = b"ICE";
    let data = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let mut out: Vec<u8> = vec![0; data.len()];

    repeating_key_xor(key, data.as_bytes(), out.as_mut_slice());
    println!("{:x?}", out);
}


fn count_ones(in1 : u8) -> u32
{
    let mut count = 0;

    for i in 0..8
    {
        let mask = (1<<i);
        let in_anded = in1 & mask;
        if in_anded != 0
        {
            count +=1;
        }
    }

    return count;
}

fn hamming_distance(in1 : &[u8], in2 : &[u8]) -> u32
{
    assert!(in1.len() == in2.len());

    let mut dist = 0;
    for i in 0..in1.len()
    {
        dist += count_ones(in1[i] ^ in2[i]);
    }

    return dist;
}

fn base64_decode_map() -> &'static HashMap<u8 , u8> {
    static HASHMAP: OnceLock<HashMap<u8, u8>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
    let mut m = HashMap::new();
    m.insert(b'A', 0 );
    m.insert(b'B', 1 );
    m.insert(b'C', 2 );
    m.insert(b'D', 3 );
    m.insert(b'E', 4 );
    m.insert(b'F', 5 );
    m.insert(b'G', 6 );
    m.insert(b'H', 7 );
    m.insert(b'I', 8 );
    m.insert(b'J', 9 );
    m.insert(b'K', 10);
    m.insert(b'L', 11);
    m.insert(b'M', 12);
    m.insert(b'N', 13);
    m.insert(b'O', 14);
    m.insert(b'P', 15);
    m.insert(b'Q', 16); 
    m.insert(b'R', 17); 
    m.insert(b'S', 18); 
    m.insert(b'T', 19); 
    m.insert(b'U', 20); 
    m.insert(b'V', 21); 
    m.insert(b'W', 22); 
    m.insert(b'X', 23); 
    m.insert(b'Y', 24); 
    m.insert(b'Z', 25); 
    m.insert(b'a', 26); 
    m.insert(b'b', 27); 
    m.insert(b'c', 28); 
    m.insert(b'd', 29); 
    m.insert(b'e', 30); 
    m.insert(b'f', 31); 
    m.insert(b'g', 32); 
    m.insert(b'h', 33); 
    m.insert(b'i', 34); 
    m.insert(b'j', 35); 
    m.insert(b'k', 36); 
    m.insert(b'l', 37); 
    m.insert(b'm', 38); 
    m.insert(b'n', 39); 
    m.insert(b'o', 40); 
    m.insert(b'p', 41); 
    m.insert(b'q', 42); 
    m.insert(b'r', 43); 
    m.insert(b's', 44); 
    m.insert(b't', 45); 
    m.insert(b'u', 46); 
    m.insert(b'v', 47); 
    m.insert(b'w', 48); 
    m.insert(b'x', 49); 
    m.insert(b'y', 50); 
    m.insert(b'z', 51); 
    m.insert(b'0', 52); 
    m.insert(b'1', 53); 
    m.insert(b'2', 54); 
    m.insert(b'3', 55); 
    m.insert(b'4', 56); 
    m.insert(b'5', 57); 
    m.insert(b'6', 58); 
    m.insert(b'7', 59); 
    m.insert(b'8', 60); 
    m.insert(b'9', 61); 
    m.insert(b'+', 62); 
    m.insert(b'/', 63); 
        
    return m;
    })
}

fn from_base64(in1 : &[u8], out : &mut [u8])
{
    let mut in_idx = 0;
    let mut out_idx = 0;

    while in_idx < in1.len()
    {
        let state = in_idx & 0x3;

        match state
        {
            0=> 
                out[out_idx] = ( base64_decode_map().get( &in1[in_idx] ).unwrap() << 2 ) | ( base64_decode_map().get( &in1[in_idx+1] ).unwrap() >> 6 ),
            1=> 
                out[out_idx] = ( base64_decode_map().get( &in1[in_idx] ).unwrap() << 4 ) | ( base64_decode_map().get( &in1[in_idx+1] ).unwrap() >> 4 ),
            2=> 
                out[out_idx] = ( base64_decode_map().get( &in1[in_idx] ).unwrap() << 6 ) | ( base64_decode_map().get( &in1[in_idx+1] ).unwrap() ),
            _=> (),
        }
        //println!("{}",state);

        out_idx += 1;
        in_idx += if state == 2 { 2 } else { 1 };
    }
}

fn find_key_size(buf : &[u8], range_low : u32, range_high : u32) -> u32 {

    let mut results: Vec<(f32,u32)> = Vec::new();
    //new vector of tuples (reusult u32, keysize)

    for i in range_low..range_high
    {
        let buf1: &[u8] = &buf[0..i as usize]; //i want this to be a pointer to the first i bytes of buf
        let buf2: &[u8] = &buf[i as usize .. 2*i as usize]; //second i bytes of buf

        let tmp_result = hamming_distance(buf1, buf2);
        let tmp_result2 = hamming::distance(buf1, buf2);

        results.push( (tmp_result as f32 / i as f32, i) );
        //println!("{} {}", tmp_result as f32 / i as f32, i);
    }

    results.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}",results);

    return results[0].1;
}

fn get_transpose_block(buf : &[u8], block_size : u32, idx : u32) -> Vec<u8> {
    let mut block: Vec<u8> = Vec::new();

    for i in (idx..buf.len() as u32).step_by(block_size as usize)
    {
        block.push(buf[i as usize]);
    }

    return block;
}


fn apply_single_character_xor(buf : &mut [u8], char_to_apply : char)
{
    for i in buf{
        *i = *i ^ char_to_apply as u8;
    }
}

fn challenge_6()
{
    let string1 = "this is a test";
    let string2 = "wokka wokka!!!";

    println!("{}", hamming_distance(string1.as_bytes(), string2.as_bytes()));

    println!("{}", hamming::distance(string1.as_bytes(), string2.as_bytes()));

    let file_path = "/home/zacht/Documents/repos/Cryptography_Learning/Rust/Set_1/src/challenge_6_data.txt";

    let mut f = File::open(file_path).unwrap();
    let mut buffer = [0; 10];

    // read exactly 10 bytes
    f.read_exact(&mut buffer).unwrap();
    //println!("{:?}", buffer);
    
    
    let mut data = std::fs::read_to_string(file_path).unwrap();
    
    //println!("{}", data);
    remove_whitespace(&mut data);
    let data_vec = BASE64_STANDARD.decode(data).unwrap();

    //println!("{:?}", data_vec);
    
    let key_size = find_key_size(data_vec.as_slice(), 2, 19);
    let mut key_vec : Vec<u8> = Vec::new();

    for i in 0..key_size
    {
        let block = get_transpose_block(data_vec.as_slice(),key_size, i);
        //println!("Block {}:\n\r{:?}",i, block);
        key_vec.push(check_single_letter_xor_vec(block) as u8);
    }
    println!("{:?}", key_vec.iter().map(|&b| b as char).collect::<Vec<char>>());

    //let key = b"ICE";
    //let data = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let mut out: Vec<u8> = vec![0; data_vec.len()];

    //let debug_str = "Terminator X: Bring the noise";
    //repeating_key_xor(debug_str.as_bytes(), data_vec.as_slice(), out.as_mut_slice());
    //println!("{:?}", out.iter().map(|&b| b as char).collect::<Vec<char>>());

    repeating_key_xor(key_vec.as_slice(), data_vec.as_slice(), out.as_mut_slice());
    //println!("{:?}", out.iter().map(|&b| b as char).collect::<Vec<char>>());

}

fn main() {

    //challenge_1();

    //challenge_2();

    //challenge_3();

    //challenge_4();

    //challenge_5();

    challenge_6();
    
}
