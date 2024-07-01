use std::io;
use std::process::Command;
use structs::ProcDef;

pub fn get_process_info() -> ProcDef {
    let mut process_name = String::new();
    println!("Enter Process Name: ");
    io::stdin()
        .read_line(&mut process_name)
        .expect("Error: Unable to read process name from user.");
    let mut process_id = String::new();
    println!("Enter Process ID: ");
    io::stdin()
        .read_line(&mut process_id)
        .expect("Error: Unable to read process id from user.");
    let mut process_binary = String::new();
    println!("Enter Process Binary Path: ");
    io::stdin()
        .read_line(&mut process_binary)
        .expect("Error: Unable to read process binary path from user.");
    let mut process_args: Vec<String> = Vec::new();
    println!("Enter Process Arguments: ");
    let mut choice = "Y".to_string();
    while choice.trim() != "N" {
        let mut arg_value = String::new();
        println!("Enter Argument Value: ");
        io::stdin()
            .read_line(&mut arg_value)
            .expect("Error: Unable to read arguments value from user.");
        process_args.push(arg_value.trim().to_string());
        println!("Current process arguments: {:#?}", process_args);
        println!("Enter 'Y' to add more arguments else 'N' !!");
        choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error: Invalid selection from user.");
    }
    let mut process_dep_list = String::new();
    println!("Enter Process Dependencies(comma separated): ");
    io::stdin()
        .read_line(&mut process_dep_list)
        .expect("Error: Unable to read process dependencies from user.");
    let mut process_dep: Vec<String> = Vec::new();
    if process_dep_list.trim() != "" {
        process_dep = process_dep_list
            .split(',')
            .map(|str| str.trim().to_string())
            .collect();
    }
    let mut process_report = String::new();
    println!("Enter Process Report Path: ");
    io::stdin()
        .read_line(&mut process_report)
        .expect("Error: Unable to read process report path from user.");

    ProcDef {
        processName: process_name.trim().to_string(),
        processId: process_id.trim().to_string(),
        processBinary: process_binary.trim().to_string(),
        processArguments: process_args,
        processDependencies: process_dep,
        processReport: process_report.trim().to_string(),
    }
}
