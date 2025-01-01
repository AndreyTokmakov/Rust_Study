use std::env;


fn input_params()
{
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{argument}");
    }
}


fn current_dir() -> std::io::Result<()>
{
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}

fn current_exe()
{
    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}", exe_path.display()),
        Err(e) => println!("failed to get current exe path: {e}"),
    };
}

fn temp_dir() {
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
}

fn get_environment_variable_by_name(name: &String)
{
    match env::var_os(&name) {
        Some(val) => println!("{name}: {val:?}"),
        None => println!("{name} is not defined in the environment.")
    }
}

fn get_environment_variable()
{
    let varName: String = "PATH".to_owned();
    get_environment_variable_by_name(&varName);
}

fn list_environment_variables()
{
    // Print all environment variables.
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}

fn list_environment_os_variables()
{
    for (key, value) in std::env::vars_os() {
        println!("{key:?}: {value:?}");
    }
}

// https://doc.rust-lang.org/std/env
pub fn test_all()
{
    // input_params();
    // current_dir();
    // current_exe();
    // temp_dir();

    // get_environment_variable();
    // list_environment_variables();
    list_environment_os_variables();

}