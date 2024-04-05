use std::process;
use colored::Colorize;

pub fn print_error(err: String) -> ! {
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
    println!("    daili [[m | mark] | [um | unmark] <argument>]");
    println!("    daili m <argument>");
    println!("    daili unmark <argument>");
    println!("    daili mark <argument>");
    println!("    daili -h | --help");
    println!();
    println!("{}", "Documentation:".green());
    println!("    daili [-h | --help] [base | today]");

    process::exit(0);
}

pub fn show_base_help() -> !
{
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
    print_warning("Dont forget that the labels, the numbers for the tasks and the quantity are your choice!");
    println!();
    println!("This is the base, the things you would like to do everyday");
    println!("The program fetches your preferences from this file to create a today.toml file (know more about it with daili -h today)");

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
