use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;


use crate::utils;

pub fn parse_and_render(src_filepath: &str) -> Result<(), std::io::Error> {
    let src_abs_filepath = utils::abspath(src_filepath).unwrap();
    let write_abs_filepath = utils::write_filepath(&src_filepath).unwrap();
    if Path::new(write_abs_filepath.as_str()).exists() {
        fs::remove_file(&write_abs_filepath)?;
    }
    let file: File = File::open(&src_abs_filepath)?;
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    let mut directive_start = false;
    let mut directive_end = true; // this is true even when directive has never been encountered
    let mut directive_cmd: Vec<String> = Vec::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                // EOF
                if bytes == 0 {
                    break;
                }

                handle_line(
                    &line,
                    &mut directive_start,
                    &mut directive_end,
                    &mut directive_cmd,
                    &write_abs_filepath,
                )?;

                // use the same buffer for next line
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(())
}


fn handle_line(
    line: &String,
    directive_start: &mut bool,
    directive_end: &mut bool,
    directive_cmd: &mut Vec<String>,
    write_abs_filepath: &String,
) -> std::io::Result<()> {

    // check if cursor is inside a directive
    if line.starts_with("@@start") {
        *directive_start = true;
        *directive_end = false;
    } else if line.starts_with("@@end") {
        *directive_start = false;
        *directive_end = true;
        // make sure the @@end directive goes inside the vector
        directive_cmd.push(line.clone());
        
        // render when the directive ends
        // TODO: Replace this with actual stuff later
        utils::append_to_file(write_abs_filepath, &"!![THIS IS A DIRECTIVE]!!\n".to_string())?;
        
        // clear the vector storing directive commands to reuse it again
        directive_cmd.clear();
    }
    
    if !*directive_start && *directive_end && !line.starts_with("@@end") {
        utils::append_to_file(write_abs_filepath, line)?;
    } else if *directive_end && !*directive_end {
        if !line.trim().is_empty() {
            directive_cmd.push(line.clone());
        }
    }

    Ok(())
}
