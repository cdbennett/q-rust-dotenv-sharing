# Demo

Answer for [this Stack Overflow question](https://stackoverflow.com/questions/75979747/how-to-share-env-values-loaded-from-api-project-to-lib-project-in-rust).

Demonstration of sharing environment variables loaded in a `bin` crate with another `lib` crate.

Uses `dotenvy` (maintained `dotenv` fork) to load the .env file.

## Example

```text
$ cat .env 
# This is the .env file used by the API project. It's loaded in the main()
# function, and then the lib_project functions can access the environment variables defined here.
MY_NAME="Linus Torvalds"

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/api_project`
Loaded environment variables from .env file
API Project calling into the library.
Hello, Linus Torvalds from the Library Project
API Project done.
```
