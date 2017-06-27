extern crate yaml_rust;


use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::process;
use std::io::Read;
use std::io::BufReader;

use self::yaml_rust::YamlLoader;
use self::yaml_rust::yaml::Yaml;

type TableMap = HashMap<String, HashMap<String, String>>;
type TableSet = HashSet<String>;


pub fn read_config(path_to_config: &str) -> (TableMap, TableSet, TableSet, TableSet, &str) {
    let configs;

    // map with instructions for transformation
    // table -> (column -> instruction)
    let mut transformation_map: TableMap = HashMap::new();
    // tables to be transformed
    let mut tables_transform_set: TableSet = HashSet::new();
    // tables to be ignored
    let mut tables_ignore_set: TableSet = HashSet::new();
    // tables to be erased
    let mut tables_erase_set: TableSet = HashSet::new();
    let default_action: &str;  // ignore or remove

    match File::open(path_to_config) {
        Err(why) => {
            println!("Can't open file \"{}\": {}", path_to_config, why.description());
            process::exit(0x0f00);
        }
        Ok(file) => {
            let mut buffer = BufReader::new(file);
            let mut contents = String::new();

            match buffer.read_to_string(&mut contents) {
                Err(why) => {
                    println!("Can't read config file! Reason: {}", why);
                    process::exit(0x0f00);
                },
                Ok(_) => {}
            };

            configs = YamlLoader::load_from_str(contents.as_ref()).unwrap();
        },
    }

    let config = &configs[0];

    if config["transform"].is_badvalue() {
        println!("Can't find the change statement!");
        process::exit(0x0f00);
    }

    match config["transform"] {
        Yaml::Hash(ref h) => {
            for (yaml_table_name, yaml_columns) in h {
                let tname = yaml_table_name.as_str().unwrap().to_string();
                let table_name = tname.clone();

                tables_transform_set.insert(tname.clone());

                let columns = HashMap::new();
                transformation_map.insert(tname, columns);

                match *yaml_columns {
                    Yaml::Hash(ref columns) => {
                        for (yaml_column_name, yaml_func) in columns {
                            let column_name = yaml_column_name.as_str().unwrap().to_string();
                            let func = yaml_func.as_str().unwrap().to_string();

                            // ERROR: cannot borrow immutable index content as mutable
                            transformation_map.get_mut(&table_name).unwrap().insert(column_name, func);
                        }
                    },
                    _ => {}
                }
            }
        },
        Yaml::Array(_) => {
            println!("Expected dictionary, got array.");
            process::exit(0x0f00);
        },
        _ => {
            println!("Expected dictionary.");
            process::exit(0x0f00);
        }
    }

    let mut i = 0;
    loop {
        if config["ignore"][i].is_badvalue() {
            break;
        }

        tables_ignore_set.insert(config["ignore"][i].as_str().unwrap().to_string());
        i += 1;
    }

    i = 0;
    loop {
        if config["remove"][i].is_badvalue() {
            break;
        }

        tables_erase_set.insert(config["remove"][i].as_str().unwrap().to_string());
        i += 1;
    }

    if config["default"].is_badvalue() {
        default_action = "ignore";
    } else if config["default"].as_str().unwrap() == "ignore" {
        default_action = "ignore";
    } else {
        default_action = "remove";
    }

    println!("########################\r\n  Config ");

    for (str, map) in &transformation_map {
        for (str2, str3) in map {
            println!("{} - {} - {}", str, str2, str3);
        }
    }

    println!("Tables to be transformed:");
    for table in &tables_transform_set {
        println!("{}", table);
    }

    println!("Tables to be ignored:");
    for table in &tables_ignore_set {
        println!("{}", table);
    }

    println!("Tables to be removed:");
    for table in &tables_erase_set {
        println!("{}", table);
    }

    println!("Default action: {}", default_action);

    println!("########################\r\n");

    (transformation_map, tables_transform_set, tables_ignore_set, tables_erase_set, default_action)
}
