use std::process;
use colored::Colorize;

pub fn print_error(err: String) -> !
{
    eprintln!("Something went wrong: {}", err.red());
    process::exit(1);
}

pub fn print_warning(warn: &str) {
    println!("{}", warn.yellow());
}

pub fn show_help() -> !
{
    println!("{}", "Allows you to see a customizable to-do list, and mark or unmark items in there".blue());
    println!();
    println!("{}", "Usage:".green());
    println!("    daili add <label> <table>");
    println!("    daili delete <task id>");
    println!("    daili [[m | mark] | [um | unmark] <argument>]");
    println!("    daili m <argument>");
    println!("    daili unmark <argument>");
    println!("    daili mark <argument>");
    println!("    daili m -a");
    println!("    daili unmark --all");
    println!("    daili -h | --help");
    println!();
    println!("{}", "Documentation:".green());
    println!("    daili [-h | --help] [base | today | modifying_base]");

    process::exit(0);
}

pub fn show_base_help() -> !
{
    print_warning("This is an important file of the internal working of the program. There is a better way of adding and removing tasks from base within the program, but if you want to hardcode your tasks, feel free.");
    println!();
    println!("{}", "Your base.toml file should be located at $HOME/.daili/ and look something like this:".blue());
    println!();
    println!("    [essential]");
    println!("    1 = \"Paint a picture\"");
    println!("    2 = \"Study for school\"");
    println!("    3 = \"Feed the cat\"");
    println!();
    println!("    [optional]");
    println!("    4 = \"Watch anime\"");
    println!("    5 = \"Try something new\"");
    println!();
    println!("This is the base, the things you would like to do everyday");
    println!("The program fetches your preferences from this file to create a today.toml file (know more about it with daili -h today)");
    println!();
    print_warning("As explained earlier, you don't need to modify those contents, because the tool offers a more user-friendly way to do that.");
    println!("(know more about it with daili -h modifying_base)");

    process::exit(0);
}

pub fn show_today_help() -> !
{
    println!("You dont need to worry about this file, as it's handled and created automatically by the cli");

    println!();

    print_warning("Your today.toml file should not ideally be touched as you can update it through the cli,");
    print_warning("but if you will, it'll be automatically created, being located at $HOME/.daili/");

    println!();

    println!("This file contains temporary data that is overwriten after a span of 24 hours from midnight (24:00 PM).");
    println!("It retains information on wether the tasks are done or not, being read and modified by the cli");
    println!("to show you and update the tasks properly");

    process::exit(0);
}

pub fn modifying_base_help() -> !
{
    println!("{}", "You can add and delete tasks from your Base file (daili -h base) using the options add and delete.".blue());
    println!();
    println!("{}-> 'daili add <the label of your task> <table you would like to add to (essential or optional)>'", "Syntax for add    ".red());
    println!("{}-> 'daili delete <the id of your task>'", "Syntax for delete ".red());
    println!();
    println!("Those modify your Base file through the tool itself, making it easier to manage them in one place.");

    process::exit(0);
}
