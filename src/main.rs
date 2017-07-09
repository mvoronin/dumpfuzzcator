extern crate clap;
extern crate crypto;
extern crate rand;
extern crate yaml_rust;

use crypto::md5::Md5;
use crypto::digest::Digest;

use rand::Rng;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

use clap::{Arg, App};

use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::yaml;

enum State {
    Skip,
    Search,
    Handle
}


fn myread<RT: BufRead, WT:Write>(input: RT, mut output: WT) {
    let mut state = State::Search;

    let mut table_name = String::from("");
    let ignore_tables: Vec<String> = vec![String::from("django_admin_log"), String::from("django_session")];
    let mut columns: Vec<String> = Vec::new();

    for o_line in input.lines() {
        let str_line = match o_line {
            Err(why) => {
                println!("Couldn't read from STDIN! Reason: {}", why);
                process::exit(0x0f00);
            },
            Ok(line) => line
        };

        let mut line: String = str_line.to_string();

        match state {
            State::Search => {
                let line: String = str_line.to_string();

                if line.len() >= 4 {
                    if &line[..4] == "COPY" {
                        let (table_name_, columns_) = parse_copy_statement(&line);
                        table_name = table_name_;
                        columns = columns_;

                        if ignore_tables.contains(&table_name) {
                            state = State::Skip;
                        } else {
                            state = State::Handle;
                        }
                    }
                }

                line.push_str("\r\n");

                match output.write(line.as_bytes()) {
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

                    line.push_str("\r\n");

                    match output.write(line.as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                } else {
                    let row = handle_row(&table_name, &columns, &str_line);

                    line.push_str("\r\n");

                    match output.write(line.as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                }
            }
            State::Skip => {
                if str_line == "\\." {
                    state = State::Search;

                    line.push_str("\r\n");

                    match output.write(line.as_bytes()) {
                        Err(why) => {
                            println!("Couldn't write to output! Reason: {}", why);
                            process::exit(0x0f00);
                        },
                        Ok(_) => {}
                    }
                }
            }
        }
    }
}


fn parse_args(args: &Vec<String>) -> (Option<String>, Option<String>) {
    let mut ifilename:Option<String> = None;
    let mut ofilename:Option<String> = None;

    let mut i = 1;

    while i+1 < args.len() {
        let arg_key = &args[i];
        let arg_value = args[i+1].clone();

        match arg_key.as_ref() {
            "-i" => { ifilename = Some(arg_value); }
            "-o" => { ofilename = Some(arg_value); }
            _ => { panic!("Unrecognized argument!"); }
        }

        i += 2;
    }

    (ifilename, ofilename)
}


fn get_random_string() -> String {
    let mut rng = rand::thread_rng();
    let rstr: String = rng.gen_ascii_chars().take(10).collect();
    // let number: u32 = rng.gen_range(0, 999999);

    rstr
}


fn get_hash(s: &str) -> String {
    let mut hasher = Md5::new();

    hasher.input_str(&s);
    hasher.result_str()
}

fn hyphenate(hash: String) -> String {
    let uuid = format!("{}-{}-{}-{}-{}", &hash[0..8], &hash[8..12], &hash[12..16], &hash[16..20], &hash[20..32]);

    uuid
}

fn handle_row(table_name: &str, columns: &Vec<String>, row: &String) -> String {
    let new_row: String;
    let vec_row: Vec<_> = row.split("\t").enumerate().collect();
    let mut vec_new_row: Vec<String> = Vec::new();

    match table_name {
        "bundle_bundle" => {
            for (i, column) in vec_row {
                match columns[i].as_ref() {
                    "first_name"  => vec_new_row.push(String::from("Jack")),
                    "last_name"  => vec_new_row.push(String::from("Johnson")),
                    _  => {
                        vec_new_row.push(column.to_string())
                    }
                }
            }

            new_row = vec_new_row.join("\t");
        },

        _ => new_row = row.clone()
    };

    new_row
}


fn parse_copy_statement(line: &str) -> (String, Vec<String>) {
    let v: Vec<&str> = line.splitn(3, ' ').collect();

    let table_name: String = String::from(v[1]);

    let columns_ind_start = *(&line.find('(').unwrap());
    let columns_ind_end = *(&line.find(')').unwrap());

    let columns_str = &line[columns_ind_start+1..columns_ind_end];

    let mut columns: Vec<String> = Vec::new();

    for c in columns_str.split(|c| c == ',' || c == ' ').filter(|c| c.len() > 0) {
        columns.push(c.to_string());
    }

    (table_name, columns)
}


fn parse_config() -> Vec<String> {
    let configs;

    let mut tables: HashMap<String,
                            HashMap<String, String>> = HashMap::new();
    let mut skip_tables: Vec<String> = Vec::new();

    match File::open("config.yml") {
        Err(why) => panic!("Couldn't open file \"{}\": {}", "config.yml", why.description()),
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer);
            configs = YamlLoader::load_from_str(buffer.as_ref()).unwrap();
            let config = &configs[0];
        },
    }

    let config = &configs[0];

    if config["tables"].is_badvalue() {
        panic!("Rules for translation wasn't found.")
    }

    match config["tables"] {
        yaml::Yaml::Hash(ref h) => {
            for (yaml_table_name, yaml_columns) in h {
                let tname = yaml_table_name.as_str().unwrap().to_string();
                let table_name = tname.clone();

                let columns = HashMap::new();
                tables.insert(tname, columns);

                match *yaml_columns {
                    yaml::Yaml::Hash(ref columns) => {
                        for (yaml_column_name, yaml_func) in columns {
                            let column_name = yaml_column_name.as_str().unwrap().to_string();
                            let func = yaml_func.as_str().unwrap().to_string();

                            // cannot borrow immutable index content as mutable
                            tables.get_mut(&table_name).unwrap().insert(column_name, func);
                        }
                    },
                    _ => {}
                }
            }
        },
        yaml::Yaml::Array(ref v) => {
            panic!("Expected dictionary, get array");
        },
        _ => {
            panic!("Expected dictionary");
        }
    }
    // for (table_name, column_map) in config["tables"] {
    //      println!("{:?}", table_name);
    // }

    let mut i = 0;
    loop {
        // println!("{:?}", config["skip"].is_badvalue());
        // println!("{:?}", config["skip"][i].is_badvalue());
        if config["skip"][i].is_badvalue() {
            break;
        }

        skip_tables.push(config["skip"][i].as_str().unwrap().to_string());
        i += 1;
    }

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(config).unwrap();  // dump the YAML object to a String
    }

    skip_tables
}

fn main() {
    let matches = App::new("rewriter")
        .version("1.0")
        .about("PG SQL dump obfuscator!")
        .author("Michael Voronin")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
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

    let path_config = matches.value_of("config").unwrap();
    println!("{}", path_config);

    let mut owrp_ifile:Option<File> = None;
    let mut owrp_ofile:Option<File> = None;

    let owrp_ifilename = matches.value_of("input");
    let owrp_ofilename = matches.value_of("output");

    parse_config();

    if let Some(filename) = owrp_ifilename {
        match File::open(&filename) {
            Err(why) => {
                println!("Couldn't open the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { owrp_ifile = Some(file) },
        };
    }

    if let Some(filename) = owrp_ofilename {
        match File::create(&filename) {
            Err(why) => {
                println!("Couldn't create the file \"{}\". Reason: {}", filename, why.description());
                process::exit(0x0f00);
            },
            Ok(file) => { owrp_ofile = Some(file) },
        }
    }

    match owrp_ifile {
        Some(ifile) => {
            let ifilebuf = BufReader::new(&ifile);

            match owrp_ofile {
                Some(ofile) => {
                    myread(ifilebuf, ofile);
                },
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();

                    myread(ifilebuf, stdout_handle);
                }
            }
        },
        None => {
            let stdin = io::stdin();
            let stdin_handle = stdin.lock();

            match owrp_ofile {
                Some(ofile) => {
                    myread(stdin_handle, ofile);
                },
                None => {
                    let stdout = io::stdout();
                    let stdout_handle = stdout.lock();

                    myread(stdin_handle, stdout_handle);
                }
            }
        }
    }
}
