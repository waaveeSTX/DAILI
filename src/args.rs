use crate::warning;

pub fn handle(args: &[String]) -> Result<(bool, String), String>
{
    if args.len() == 1
    {
        return Ok((false, "none".to_string()));
    }
    
    if args[1].as_str() == "-h" || args[1].as_str() == "--help"
    {
        if args.len() >= 3
        {
            match args[2].as_str()
            {
                "base"  => warning::show_base_help(),
                "today" => warning::show_today_help(),
                   _    => warning::print_error(format!("{} is not a valid sub-option of flag -h (or --help)", args[2].as_str()))
            }
        }

        warning::show_help()
    }

    if args.len() < 3
    {
        return Err("There are too few args. Check the right usage with \"daili -h\"".to_string());
    }

    let mark: bool = match args[1].as_str()
    {
        "m"  | "mark"   => true,
        "um" | "unmark" => false,
         _   => return Err(format!("{} is not a valid argument!", args[1].as_str()))
    };

    let item_to_do_action_in: String = args[2].to_string();

    Ok((mark, item_to_do_action_in))
}
