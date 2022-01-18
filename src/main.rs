use structopt::StructOpt;
mod input;
use std::error::Error;
mod protocols;
mod decorators;
mod services;
use std::env;

fn main() -> Result<(), Box<dyn Error>>{
    if !protocols::checkenv::check_env() {
        Err("ENV variables not declared properly")?
    }
    modify_env_with_flags();
    let env_manager = protocols::EnvManager::init();
    let mut dir_tree = protocols::DirTree::init(input::Cli::from_args().dir, env_manager);
    dir_tree.gen();
    Ok(())
}

fn modify_env_with_flags() {
    let set_env: &str = &input::Cli::from_args().set_env.clone();
    println!("modify!! {}", set_env);
    if set_env != "" && check_valid_set_env(set_env.to_string()){
        set_env_var(&set_env);
    }
}


fn set_env_var(env_string: &str){
    let string_vec: Vec<&str> = env_string.split("=").collect();
    env::set_var(string_vec[0].to_uppercase(), string_vec[1].to_uppercase());
}

fn check_valid_set_env(env_input: String) -> bool{
    let used_positions: Vec<String> = protocols::checkenv::get_used_positions();
    let string_vec: Vec<&str> = env_input.split("=").collect();
    if string_vec.len() != 2{
        println!("ERROR: invalid flag variable for --set-env");
        return false;
    }
    return protocols::checkenv::check_env_var(string_vec[0], string_vec[1], &used_positions);
}



#[cfg(test)]
mod test_suite;