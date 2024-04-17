use crate::warning;

pub fn handle(args: &[String]) -> Result<(bool, Vec<String>), String>
{
    if args.len() == 1
    {
        return Ok((false, vec!["none".to_string()]));
    }

    if args[1].as_str() == "-h" || args[1].as_str() == "--help"
    {
        if args.len() >= 3
        {
            match args[2].as_str()
            {
                "base"  => warning::show_base_help(),
                "today" => warning::show_today_help(),
                _    => warning::print_error(format!("{} is not a valid sub-option of flag -h (aka --help)", args[2].as_str()))
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

    let mut tasks_to_operate_on: Vec<String> = vec![];

    let all = match args[2].as_str()
    {
        "-a" | "--all" => true,
        _              => false
    };

    if !all
    {
        for arg in &args[2..args.len()]
        {
            if let Err(_) = arg.parse::<i32>()
            {
                return Err("You can only insert numbers or \"-a/--all\" as task ids".to_string());
            }

            tasks_to_operate_on.push(arg.to_string());
        }
    }
    else
    {
        tasks_to_operate_on = vec!["all".to_string()];
    }

    Ok((mark, tasks_to_operate_on))
}
