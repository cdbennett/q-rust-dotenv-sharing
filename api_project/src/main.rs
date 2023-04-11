fn main() {
    match dotenvy::dotenv() {
        Ok(_) => println!("Loaded environment variables from .env file"),
        Err(err) => {
            let cwd = std::env::current_dir().unwrap();
            let cwd = cwd.as_path().to_string_lossy();
            println!("Can't load .env file (CWD is {cwd}): {err:?}");
        }
    }

    println!("API Project calling into the library.");

    lib_project::say_hello();
    lib_project::do_stuff();

    println!("API Project done.");
}
