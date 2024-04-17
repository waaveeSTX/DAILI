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

    let (mark, mut tasks_to_operate_on) = match args::handle(&args)
    {
        Ok((mark, tasks_to_operate_on)) => (mark, tasks_to_operate_on),
        Err(err) => warning::print_error(err)
    };

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

    let mut action_done: bool = false;

    if tasks_to_operate_on == vec!["all".to_string()]
    {
        tasks_to_operate_on = today.essential.keys()
                                             .map(|k| k.clone())
                                             .chain(today.optional.keys()
                                                                  .map(|k| k.clone()))
                                                                  .collect();
    }

    for task_id in tasks_to_operate_on
    {
        let done = task::set_done_to(&mut today, &task_id, mark).unwrap_or_else(|err|
            warning::print_error(err));

        if done
        {
            action_done = true;
        }
    }

    let mut new_today_contents = String::new();
    if action_done
    {
        new_today_contents = files::get_new_contents(&today).unwrap_or_else(|err|
            warning::print_error(err));
    }

    if new_today_contents != ""
    {
        println!();

        task::print_tasklist_from(&today);

        files::overwrite(&paths.today, &new_today_contents).unwrap_or_else(|err|
            warning::print_error(err));
    }
}
