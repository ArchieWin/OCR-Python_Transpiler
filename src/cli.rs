use std::io::{self, BufWriter, Write};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader};
mod transpiler;


pub fn init_cli() {

    let mut file_path = String::new();
    let mut transpiled_file_path = String::new();

    //Ask the user for the file they wish to transpile
    loop {
        
        println!("Enter the path of the file to be transpiled to python code: ");
        io::stdout().flush().expect("Could not flush.");

        let _result = io::stdin().read_line(&mut file_path).expect("Could not read line.");

        file_path = file_path.trim().to_string().replace("\"", "");

        println!("Enter the file path for the transpiled file: ");
        io::stdout().flush().expect("Could not flush.");

        let _result = io::stdin().read_line(&mut transpiled_file_path).expect("Could not read line.");

        transpiled_file_path = transpiled_file_path.trim().to_string().replace("\"", "");

        if read_file(&file_path, &transpiled_file_path) {
            break;
        }

        file_path.clear();
    }

}

fn read_file(file_path: &String, transpiled_file_path: &String) -> bool {

    //temporary
    let file_path = String::from("C:\\Users\\marky\\OneDrive\\Desktop\\test.txt");
    let transpiled_file_path = String::from("C:\\Users\\marky\\OneDrive\\Desktop\\transpiled_file.txt");


    let file_result = File::open(file_path);

    let file = match file_result {
        Ok(file) => file,
        Err(e) => {
            println!("Could not read the file!, please try again.");
            println!("Error: {e}");
            return false;
        }
    };

    let transpiled_file_result = OpenOptions::new()
        .write(true)
        .append(false)
        .create(false)
        .open(transpiled_file_path);

    let transpiled_file = match transpiled_file_result {
        Ok(transpiled_file) => transpiled_file,
        Err(e) => {
            println!("Could not write to the file!, please try again.");
            println!("Error: {e}");
            return false;
        }
    };



    read_file_lines(file, transpiled_file);

    return true;
}

fn read_file_lines(file: File, transpiled_file: File) {

    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(transpiled_file);

    for line in reader.lines() {

        let line = line.unwrap();

        let translated_line = transpiler::translate_line(&line);

        writeln!(writer, "{}", translated_line).expect("Could not write line.");
    }

    match writer.flush() {
        Ok(_) => println!("Transpiled to file successfully!"),
        Err(_) => println!("Could not flush file writer.")
    };

}