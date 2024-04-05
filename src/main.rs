use std::process;
use std::env;

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

use serde;
use colored::Colorize;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let (mark, item_do_do_action_in) = match args::handle(&args)
    {
        Ok((mark, item_to_do_action_in)) => (mark, item_to_do_action_in),
        Err(err) => warning::print_error(err)
    };

    // Seting up needed variables
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

    let base: Base = match Base::from_str(&contents.base)
    {
        Ok(base) => base,
        Err(err) =>
        {
            warning::print_warning(&err);
            println!("{}", "Learn how to create a base.toml file using \"daili -h base\"".blue());
            process::exit(1);
        }
    };

    let mut today: Today = match Today::handle(&contents, &paths.today, &base)
    {
        Ok(today) => today,
        Err(err)  => warning::print_error(err)
    };

    println!("{}", today.date.yellow());
    task::print_tasklist_from(&today);
    
    let new_today_contents: String = task::set_done_to(&mut today, &item_do_do_action_in, mark).unwrap_or_else(|err|
                                        warning::print_error(err));

    if new_today_contents != ""
    {
        println!();

        task::print_tasklist_from(&today);

        files::overwrite(&paths.today, &new_today_contents).unwrap_or_else(|err|
            warning::print_error(err));
    }
}
