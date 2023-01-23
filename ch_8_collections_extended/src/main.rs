/* 
—————————————————————————— Constraints ——————————————————————————
>> Use Hashmaps & Vectors

—————————————————————————— Feature #1 ——————————————————————————
create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”

Command | Add
Data 	| Employee: (Name), Department: (Name)


—————————————————————————— Feature #2 ——————————————————————————
Retrieve a list of all people in a department or
all people in the company by department, sorted alphabetically.

Command    | All, List
Input Data | Employee: (Name), Department: (Name)


—————————————————————————— Feature #3 ——————————————————————————
Exit the program when the user is finished
Command    | Quit

*/

/*  
—————————————————————————— Tasks —————————————————————————— 

[X] Program loop to keep it running until User ends it

[ ] CLI - parse user input for commands & input data
TODO: Do something about the weird formatting on the CLI after entering a command.

[X] Data structures(s) to store: Commands

[X] Data structures(s) to: store multiple Employees, multiple Departments for 1x Company

 */

 use std::{io, collections::HashMap};

fn main() {
	println!("Welcome! <3 \n");

	Command::help();

	let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    
	'mainLoop: loop {
		let mut input = String::new();

		io::stdin().read_line(&mut input).expect("error: input failed \n");

		match Command::from_input(&input) {
			Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
			Some(Command::List(dept)) => Command::list(&dept, &employees),
			Some(Command::All) => Command::all(&employees),
			Some(Command::Help) => Command::help(),
			Some(Command::Quit) => break 'mainLoop,
			None => println!("I don't recognize that entry! \n")
		}
	}

}

enum Command {
	Add { dept: String, name: String },
	List(String),
	All,
	Quit,
	Help
}

impl Command {
	fn from_input(s: &str) -> Option<Self> {
		let words: Vec<&str> = s.trim().split_whitespace().collect();

		// each pattern is a destructured array. ["All"] is looking for a 1-element length array, w/ the first value being All
		match words.as_slice() {
			["All"] => Some(Command::All), // looks for a 1-element array w/ first word being All
			["Quit"] => Some(Command::Quit), // looks for a 1-element array w/ first word being Quit
			["Help"] => Some(Command::Help), // looks for a 1-element array w/ first word being Help
			["List", dept] => Some(Command::List(dept.to_string())), // looks for a 2-element array w/ first word being List
		 	["Add", name, "to", dept] => Some(Command::Add { dept: dept.to_string(), name: name.to_string() }), // looks for a 4-element array w/ first Add, etc
			_ => None,
		}
	}

	fn list(department: &str, employees: &HashMap<String, Vec<String>>) {
		match employees.get(department) {
			Some(names) =>  {
				let mut names = names.clone();
				names.sort();
				for name in names {
					println!("{}: {}", department, name);
				}
			},
			None => println!("Nobody works in this department. \n")
		}
	}

	fn all(employees: &HashMap<String, Vec<String>>) {
		for department in employees.keys() {
			Command::list(department, employees)
		}
	}

	fn help() {
		println!("COMMANDS ");
		println!("Type 'Add <name> to <department>' to add an employee");
		println!("Type 'List <department>' to list the employees of a department");
		println!("Type 'All' to list all employees by department");
		println!("Type 'Quit' to quit \n");
	}
}




// Departments = Hashmap[Department, Vector[Employee X, Employee Y, etc]]
