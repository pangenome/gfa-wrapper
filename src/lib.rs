extern crate libc;

use libc::c_char;
use libc::c_int;
use std::ffi::CStr;

// use std::fs::File;
// use std::io::Read;
// use std::str;


use gfa::{gfa::GFA, parser::GFAParser};
use ffi_convert::{CReprOf, CStringArray };




#[no_mangle]
pub extern "C" fn consume_gfa_file (pointer_path:  *const c_char )  -> CStringArray {


    let  gfa_path =  unsafe{ &CStr::from_ptr(pointer_path)};
    let parser = GFAParser::new();
    let str_slice: &str = gfa_path.to_str().unwrap();
    let gfa: GFA<usize, ()> = parser.parse_file(str_slice).unwrap();
    let mut all_sequences : Vec<String>  = Vec::new();

    for i in  gfa.segments
    {
        let _ = match String::from_utf8(i.sequence) {
            Ok(v) => all_sequences.push(v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

    }

    let result = CStringArray::c_repr_of(all_sequences).expect("could not convert !");
    result
}





#[no_mangle]
pub extern "C" fn consume_gfa_file_raw (pointer_path:  *const c_char, number_of_items: *mut c_int)  -> *const *const c_char   {


    let  gfa_path =  unsafe { &CStr::from_ptr(pointer_path) };
    let parser = GFAParser::new();
    let str_slice: &str = gfa_path.to_str().unwrap();
    let gfa: GFA<usize, ()> = parser.parse_file(str_slice).unwrap();
    let mut all_sequences : Vec<String>  = Vec::new();

    for i in  gfa.segments
    {
        let _ = match String::from_utf8(i.sequence) {
            Ok(v) => all_sequences.push(v),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

    }

    let result = CStringArray::c_repr_of(all_sequences).expect("could not convert !");
    unsafe {core::ptr::write(number_of_items, result.size as c_int) ;
        result.data
    }

}
