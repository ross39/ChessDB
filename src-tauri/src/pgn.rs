
use std::{fs::File, io::Write};
use pgn_reader::BufferedReader;


//function to create a given text file
#[tauri::command]
pub fn create_pgn(pgn: String) -> Result<(), String> {
    let mut file = File::create("test.pgn").unwrap();
    file.write_all(pgn.as_bytes()).unwrap();
    Ok(())
}






