

pub fn parse_copy_statement(line: &str) -> (String, Vec<String>) {
    let v: Vec<&str> = line.split(' ').collect();
    let mut columns: Vec<String> = Vec::new();

    let table_name = v[1].to_string();

    let ind_start_columns = *(&line.find('(').unwrap());
    let ind_end_columns = *(&line.find(')').unwrap());

    let columns_str = &line[ind_start_columns+1..ind_end_columns];

    for c in columns_str.split(|c| c == ',' || c == ' ').filter(|c| c.len() > 0) {
        columns.push(c.to_string());
    }

    (table_name, columns)
}
