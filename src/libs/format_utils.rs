use colored::*;
use prettytable::{Table, Row, Cell};

pub fn colored_alias(alias: String) -> String{
    if alias.starts_with("r") {
        return format!("{}", alias.blue());
    } else if alias.starts_with("mr") {
        return format!("{}", alias.green());
    } else if alias.starts_with("m") {
        return format!("{}", alias.red());
    } else {
        return format!("{}", alias.yellow());
    }
}

pub fn print_vec_as2d(values: Vec<String>, columns: usize) {
    let mut table = Table::new();
    let mut row: Vec<Cell> = Vec::new();

    // add column names
    row.push(Cell::new("ports/hosts"));
    for i in 1..(columns+1) {
        row.push(Cell::new(format!("host-{}", i).as_str()));
    }

    // add instances
    for i in 0..values.len() {
        if i % columns == 0 {
            table.add_row(Row::new(row.clone()));
            row.clear();
            row.push(Cell::new("port.."));
        }
        row.push(Cell::new(values[i].as_str()));
    }

    table.printstd();
}
