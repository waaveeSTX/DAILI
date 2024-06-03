use crate::warning;
use crate::action::{Action, ActionKind};

fn handle_help_argument(args: &[String])
{
    // In the case of the flag -h being used
    if args[1].as_str() == "-h" || args[1].as_str() == "--help"
    {
        // Sub-options of the flag -h (documentation for base and today files)
        if args.len() >= 3
        {
            match args[2].as_str()
            {
                "base"           => warning::show_base_help(),
                "today"          => warning::show_today_help(),
                "modifying_base" => warning::modifying_base_help(),
                _                => warning::print_error(format!("{} is not a valid sub-option of flag -h (aka --help)", args[2].as_str()))
            }
        }

        warning::show_help()
    }
}

pub fn get_action(args: &[String]) -> Result<Action, String>
{
    // In the case of having no arguments provided
    if args.len() == 1
    {
        // Returning a function with the kind None so it can be handled later
        return Ok(Action { affirmative: true, kind: ActionKind::None })
    }

    handle_help_argument(args);

    // In the case of there being less than 2 arguments provided
    if args.len() < 3
    {
        return Err("There are too few args. Check the right usage with \"daili -h\"".to_string());
    }

    let action: Action = match args[1].as_str()
    {
        "m"  | "mark"     => Action { affirmative: true,  kind: ActionKind::Marking },
        "um" | "unmark"   => Action { affirmative: false, kind: ActionKind::Marking },
        "add"             => Action { affirmative: true,  kind: ActionKind::ModifyingBase },
        "delete"          => Action { affirmative: false, kind: ActionKind::ModifyingBase },
        _                 => return Err(format!("{} is not a valid argument!", args[1].as_str()))
    };

    Ok(action)
}
