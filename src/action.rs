use std::{fs, path::PathBuf};

use crate::{base::Base, extracting, task, today::Today};

// enum that represents the diferent types an action can take, just for identification
pub enum ActionKind
{
    Marking,
    ModifyingBase,
    None
}

// an action. When not affirmative it does the opposite, for example looking at marking
// 'mark' is the affirmative action of kind Marking, 'unmark' being the action of the same kind but
// not affirmative
pub struct Action
{
    pub affirmative: bool,
    pub kind: ActionKind
}

fn get_tasks_to_operate_on(args: &[String]) -> Result<Vec<String>, String>
{
    let mut tasks_to_operate_on: Vec<String> = vec![];

    // Bulk marking with the -a flag
    let all = match args[2].as_str()
    {
        "-a" | "--all" => true,
        _              => false
    };

    // Bulk marking with manual picking
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

    Ok(tasks_to_operate_on)
}

// Given an action and a Today object, act upon the Today object with the given configuration.
// Returns wether the action was done or not, in the case of the action type being none that value
// would be false.
pub fn act(args: &[String], today: &mut Today, base: &mut Base, base_path: &PathBuf, today_path: &PathBuf, current_date: &str, action: &Action) -> Result<bool, String>
{
    let mut action_done: bool = false;

    match action.kind
    {
        ActionKind::None =>
        {
            action_done = false;
        }

        ActionKind::Marking =>
        {
            let mut tasks_to_operate_on = get_tasks_to_operate_on(args)?;

            // Getting all the tasks from today if the tasks_to_operate_on vector is populated only with
            // "all"
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
                let done = task::set_done_to(today, &task_id, action.affirmative)?;

                if done
                {
                    action_done = true;
                }
            }
        }

        ActionKind::ModifyingBase =>
        {
            if action.affirmative
            {
                let (task_text, task_table) = (args[2].to_string(), &args[3]);
                let new_task_id = (base.essential.len() + base.optional.len() + 1).to_string();

                match task_table.as_str()
                {
                    "essential" => base.essential.insert(new_task_id, task_text),
                    "optional"  => base.optional.insert(new_task_id, task_text),
                    _           => return Err("Please enter only \"essential\" or \"optional\" as the owner table".to_string())
                };
            }
            else if !action.affirmative
            {
                let task_id = args[2].to_string();

                if !base.essential.contains_key(&task_id) && !base.optional.contains_key(&task_id)
                {
                    return Err(format!("{task_id} wasn't found in neither of the tables"));
                }

                let owner_table = match base.essential.contains_key(&task_id)
                {
                    true  => "essential",
                    false => "optional"
                };

                match owner_table
                {
                    "essential" => base.essential.remove(&task_id),
                    "optional"  => base.optional.remove(&task_id),
                    _           => return Err(format!("{task_id} wasn't found in neither of the tables"))
                };
            }

            let new_base_contents: String = toml::to_string(&base).map_err(|err| err.to_string())?;

            // Writting the new base contents into the base file
            fs::write(base_path, &new_base_contents).map_err(|err| err.to_string())?;

            *today = extracting::convert_base_to_today(&base, current_date);

            // Writting the today contents from the new base into the today file
            fs::write(today_path, &toml::to_string(today).map_err(|err| err.to_string())?).map_err(|err| err.to_string())?;

            action_done = true;
        }
    }

    Ok(action_done)
}
