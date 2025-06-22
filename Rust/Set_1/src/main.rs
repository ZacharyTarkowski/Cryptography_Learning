
use std::{fmt::Write, num::ParseIntError};

use std::collections::HashMap;
use std::sync::OnceLock;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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

fn repeating_key_xor(key : &[u8])
{

}

fn challenge_5()
{
    let key = b"ICE";
}

fn main() {

    challenge_1();

    challenge_2();

    challenge_3();

    challenge_4();

    challenge_5();
    
}
