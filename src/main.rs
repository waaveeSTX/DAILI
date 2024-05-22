use std::{env, fs};

mod action;
mod args;
mod warning;
mod paths;
mod files;
mod base;
mod today;
mod extracting;
mod task;

use paths::Paths;
use files::{Files, Contents};
use base::Base;
use today::Today;

use chrono::prelude::*;
use serde;
use colored::Colorize;

use action::Action;

fn main()
{
    let current_date = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    let args: Vec<String> = env::args().collect();

    let action: Action = args::get_action(&args).unwrap_or_else(|err| warning::print_error(err));

    let paths: Paths = match Paths::new()
    {
        Ok(paths) => paths,
        Err(err) => warning::print_error(err),
    };

    let mut files: Files = match Files::new(&paths)
    {
        Ok(files) => files,
        Err(err) => warning::print_error(err),
    };

    let contents: Contents = match Contents::new(&mut files)
    {
        Ok(contents) => contents,
        Err(err) => warning::print_error(err),
    };

    let mut base: Base = match Base::new(&contents.base)
    {
        Ok(base) => base,
        Err(err) => warning::print_error(err)
    };

    let mut today: Today = match Today::handle(&contents, &paths.today, &base, &current_date)
    {
        Ok(today) => today,
        Err(err)  => warning::print_error(err)
    };

    // Printing before making a change
    println!("{}", today.date.yellow());
    task::print_tasklist_from(&today);

    let action_done: bool = action::act(&args, &mut today, &mut base, &paths, &current_date, action).unwrap_or_else(|err| warning::print_error(err));

    // This is only changed if the action was done, being a empty string otherwise
    let new_today_contents = match action_done
    {
        true  => toml::to_string(&today).unwrap_or_else(|err| warning::print_error(err.to_string())),
        false => String::new()
    };

    // If the action was done, print the new list and update its contents as well
    if new_today_contents != ""
    {
        println!();
        println!("-/--/--/--/--/--/--/--/--/--/--/--/--/--/--/-");
        println!();

        task::print_tasklist_from(&today);

        fs::write(&paths.today, &new_today_contents).map_err(|err| err.to_string()).unwrap_or_else(|err|
            warning::print_error(err));
    }
}
