extern crate clap;
extern crate crypto;
extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

use clap::{Arg, App};

use self::parsers::config::read_config;

type TableMap = HashMap<String, HashMap<String, String>>;
type TableSet = HashSet<String>;


mod parsers;
mod translator;


enum State {
    Search,
    Handle,
    Ignore,
    Remove
}


fn transform<RT: BufRead, WT:Write>(input: RT, mut output: WT,
                                    transformation_map: TableMap,
                                    tables_transform_set: TableSet,
                                    tables_ignore_set: TableSet,
                                    tables_erase_set: TableSet,
                                    default_action: &str) {

    let mut state = State::Search;

    let mut current_table_name = String::from("");
    let mut current_table_columns: Vec<String> = Vec::new();

    for o_line in input.lines() {
        let str_line = match o_line {
            Err(why) => {
                println!("Couldn't read from STDIN! Reason: {}", why);
                process::exit(0x0f00);
            },
            Ok(line) => line
        };

        match state {
            State::Search => {
                let mut current_line: String = str_line.to_string();

                if current_line.len() >= 4 {
                    if &current_line[..4] == "COPY" {
                        let (table_name_, columns_) = parsers::sql::parse_copy_statement(&current_line);
                        current_table_name = table_name_;
                        current_table_columns = columns_;

                        if tables_ignore_set.contains(&current_table_name) {
                            state = State::Ignore;
                        } else if tables_erase_set.contains(&current_table_name) {
                            state = State::Remove;
                        } else {
                            state = State::Handle;
                        }
                    }
                }

                current_line.push_str("\r\n");

                match output.write(current_line.as_bytes()) {
                    Err(why) => {
                        println!("Couldn't write to output! Reason: {}", why);
                        process::exit(0x0f00);
                    },
                    Ok(_) => {}
                }
            },
            State::Handle => {
                if str_line == "\\." {
                    state = State::Search;

                    match output.write(concat!("\\.", "\r\n").as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                } else {
                    // table_name - current table name of table which we are processing
                    // columns - columns' names
                    // str_line - current line we process
                    // tablesmap - ???
                    let mut row = translator::transform_row(
                        &current_table_name,
                        &current_table_columns,
                        &str_line,
                        &transformation_map,
                        default_action);

                    row.push_str("\r\n");

                    match output.write(row.as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                }
            }
            State::Ignore => {
                // do nothing until we get an end of a table
                // что-то я не понял почему я пишу каждый раз конец таблицы вместо данных
                if str_line == "\\." {
                    state = State::Search;

                    match output.write(concat!("\\.", "\r\n").as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                }
            }
            State::Remove => {
                if str_line == "\\." {
                    state = State::Search;
                }
            }
        }
    }
}


fn main() {
    let matches = App::new("dumpfuzzcator")
        .version("1.0")
        .about("PG SQL dump obfuscator!")
        .author("Michael Voronin")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a config file")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file")
            .takes_value(true))
        .get_matches();

    let path_to_config = matches.value_of("config").unwrap();

    let mut opt_ifile:Option<File> = None;
    let mut opt_ofile:Option<File> = None;

    let opt_ifilename = matches.value_of("input");
    let opt_ofilename = matches.value_of("output");

    let (transformation_map,
        tables_transform_set,
        tables_ignore_set,
        tables_erase_set,
        default_action) = read_config(path_to_config);

    // if path to the input file was passed then we open it.
    if let Some(filename) = opt_ifilename {
        match File::open(&filename) {
            Err(why) => {
                println!("Couldn't open the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { opt_ifile = Some(file) },
        };
    }

    // if path to the output file was passed then we create it.
    if let Some(filename) = opt_ofilename {
        match File::create(&filename) {
            Err(why) => {
                println!("Couldn't create the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { opt_ofile = Some(file) },
        }
    }

    match opt_ifile {
        // if we are opening the input file
        Some(ifile) => {
            let ifilebuf = BufReader::new(&ifile);

            match opt_ofile {
                // we opened the input file and the output file
                Some(ofile) => {
                    transform(ifilebuf, ofile,
                              transformation_map,
                              tables_transform_set,
                              tables_ignore_set,
                              tables_erase_set,
                              default_action);
                },
                // we opened the input file, for the output we will use the output stream
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();

                    transform(ifilebuf, stdout_handle,
                              transformation_map,
                              tables_transform_set,
                              tables_ignore_set,
                              tables_erase_set,
                              default_action);
                }
            }
        },
        // we are going to use the input stream for the input data
        None => {
            let stdin = io::stdin();
            let stdin_handle = stdin.lock();

            match opt_ofile {
                // the input stream for input data and the output file for the output data
                Some(ofile) => {
                    transform(stdin_handle, ofile,
                              transformation_map,
                              tables_transform_set,
                              tables_ignore_set,
                              tables_erase_set,
                              default_action);
                },
                // the input strean for input data and the output strean for the output data
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();

                    transform(stdin_handle, stdout_handle,
                              transformation_map,
                              tables_transform_set,
                              tables_ignore_set,
                              tables_erase_set,
                              default_action);
                }
            }
        }
    }
}
