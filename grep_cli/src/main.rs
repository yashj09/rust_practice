use std::env; // use args_os for accepting unicode
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    // let content=fs::read_to_string(file_path)
    // .expect("Fail to read the file content"); simpler, no error handling
    let content = fs::read_to_string(file_path);
    let file = match content {
        Ok(file) => {
            println!("all good");
            file
        }
        Err(error) => {
            println!("getting error  : {error}");
            return;
        }
    };
    dbg!(&args);
    println!("argument 1 {}\nargument 2 {}", query, file_path);
    println!("File content are \n {}", file);
}
