extern crate rand;

use std::collections::HashMap;
use std::process;


type TableMap = HashMap<String, HashMap<String, String>>;


pub mod generators;


pub fn transform_row(current_table: &str, current_table_columns: &Vec<String>, row: &String,
                     transformation_map: &TableMap, default_action: &str) -> String {
    let new_row: String;
    let vec_current_row: Vec<_> = row.split("\t").enumerate().collect();
    let mut vec_new_row: Vec<String> = Vec::new();

    // println!("{:?}", columns);
    // println!("{:?}", row);
    // println!("{:?}", vec_row);

    if transformation_map.get(current_table) == None {
        if default_action == "ignore" { return row.clone() }
        else if default_action == "remove" { return String::from("") };
        println!("Default action value is invalid.");
        process::exit(0x0f00);
    } else {
        let columns_transformations_map = transformation_map.get(current_table).unwrap();

        // println!("=== COLUMNS MAP ===");
        // println!("{:?}", columns_map);
        // println!("=============================================================");

        for (current_column_index, current_column_data) in vec_current_row {
            if columns_transformations_map.get(&current_table_columns[current_column_index]) == None {
                // if there is no instructions of transformation for current column, just write it as is
                // println!("not in map: {:?} not in {:?} ", columns[i], columns_map);
                vec_new_row.push(current_column_data.to_string());
                // println!("vec_new_row {:?}", vec_new_row);
            } else {
                // println!("in map: {:?} in {:?} ", columns[i], columns_map);
                // column transformation instruction
                let inst = columns_transformations_map.get(&current_table_columns[current_column_index]).unwrap().clone();
                let mut new_value: String = String::from("");
                // println!("{:?}", column_update_instruction);
                if inst.len() > 0 {
                    if &inst[..1] == "!" {
                        if inst.len() > 7 {
                            if &inst[1..7] == "random" {
                                println!("{}", &inst[1..7]);
                                let inst = &inst[8..inst.len()-1];
                                println!("=> {}", inst);

                                if inst == "int" {
                                    new_value = generators::generate_int(4);
                                } else if inst == "date" {
                                    // new_value = generators::generate_date();
                                } else if inst == "email" {
                                    new_value = generators::generate_email();
                                } else if inst == "firstname" {
                                    new_value = generators::generate_male_firstname();
                                } else if inst == "lastname" {
                                    new_value = generators::generate_lastname();
                                } else if &inst[0..6] == "string" {
                                    let range = &inst[..5];
                                    let str_range_start = &inst[7..inst.find("-").unwrap()];
                                    let range_start = str_range_start.parse::<i32>().unwrap();
                                    let str_range_end = &inst[inst.find("-").unwrap()+1..];
                                    let range_end = str_range_end.parse::<i32>().unwrap();
                                    println!("{}-{}", range_start, range_end);
                                    new_value = generators::generate_string(range_start as usize);
                                }
                            }
                        }
                    }
                }
                vec_new_row.push(new_value);
                // println!("vec_new_row {:?}", vec_new_row);
            }
        }

        new_row = vec_new_row.join("\t");
    }

    new_row
}
