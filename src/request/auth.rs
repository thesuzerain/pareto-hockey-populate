use std::{io, fs};
use lazy_static::lazy_static;

// Adds apiKey field with authentication code to existing HTTP request
// eg: "http://something.com/endpoint?" -> "http://something.com/en0dpoint?&apiKey=xxxxx"
pub fn with_auth_code_field(s : String ) -> String {
    lazy_static!{
        static ref AUTH_CODE : String = load_auth_code().expect("Could not find auth key saved in 'auth.txt' file.");
    }
    format!("{s}&apiKey={:}",*AUTH_CODE)
} 

// Loads API key from file to String
pub fn load_auth_code() -> Result<String, io::Error> {
    Ok(fs::read_to_string(crate::AUTH_FILE_LOCATION)?) 
}