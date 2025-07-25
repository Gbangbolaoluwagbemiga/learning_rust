use std::fs;
use std::io::Error;

fn get_errors(logs_data: String) -> String {
    let mut errors = vec![];

    let logs = logs_data.split("\n");
    for log in logs {
        if log.starts_with("ERROR") {
            errors.push(log);
        }
    }
    errors.join("\n")
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = get_errors(text);
    fs::write("error.txt", errors)?;

    Ok(())

    // let text = fs::read_to_string("logs.txt").expect("There is an error with the path");
    // let errors = get_errors(text);
    // fs::write("error.txt", errors).expect("Failed to write errors to file");

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let errors = get_errors(text);
    //         match fs::write("error.txt", errors) {
    //             Ok(_) => println!("Wrote error.txt"),
    //             Err(e) => println!("Failed to write error.txt: {}", e),
    //         }
    //     }
    //     Err(e) => println!("Failed to read logs.txt: {}", e),
    // }
}
