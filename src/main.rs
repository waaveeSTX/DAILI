use std::{env, fs};

mod action;
mod args;
mod warning;
mod daili_path;
mod file_container;
mod base;
mod today;
mod container;
mod extracting;
mod task;

use chrono::prelude::*;
use serde;
use colored::Colorize;
use action::Action;

use base::Base;
use today::Today;
use container::Container;

fn main()
{
    // 1 - Initializing daili_path, current_date and args; extracting the action
    let daili_path = daili_path::get_and_create_daili_path().unwrap_or_else(|err| warning::print_error(err));

    let current_date = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let args: Vec<String> = env::args().collect();

    let action: Action = args::get_action(&args).unwrap_or_else(|err| warning::print_error(err));


    // 2 - Initializing containers
    let mut base  = Container::<Base>::init(&daili_path.join("base.toml"), None, None).unwrap_or_else(|err| warning::print_error(err));
    let mut today = Container::<Today>::init(&daili_path.join("today.toml"), Some(&base.object), Some(&current_date)).unwrap_or_else(|err| warning::print_error(err));


    // 3 - Printing the previous state and updating it by acting upon it with the configuration
    //   extracted
    println!("{}", today.object.date.yellow());
    task::print_tasklist_from(&today.object);

    let action_done: bool = action::act(&args, &mut today.object, &mut base.object, &base.file.path, &today.file.path, &current_date, &action).unwrap_or_else(|err| warning::print_error(err));


    // 4 - Displaying the new state if it is changed
    let new_today_contents = match action_done
    {
        true  => toml::to_string(&today.object).unwrap_or_else(|err| warning::print_error(err.to_string())),
        false => String::new()
    };

    if new_today_contents != ""
    {
        println!();
        println!("-/--/--/--/--/--/--/--/--/--/--/--/--/--/--/-");
        println!();

        task::print_tasklist_from(&today.object);

        fs::write(today.file.path, &new_today_contents).map_err(|err| err.to_string()).unwrap_or_else(|err| warning::print_error(err));
    }
}
