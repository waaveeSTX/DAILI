use std::collections::BTreeMap;
use crate::{extracting, today::Today, warning};
use serde::{Serialize, Deserialize};

// The bare structure that is a task, without any identification of the table it belongs to,
// just the text for it and the completion status.
#[derive(Deserialize, Serialize)]
pub struct UnidentifiedTask
{
    pub text: String,
    pub done: bool
}

// The task but with that identification
// owner_table must be either "essential" or "optional"
pub struct IdentifiedTask<'a>
{
    pub task: &'a UnidentifiedTask,
    pub owner_table: &'a str
}

// The list of tasks that will be extracted from a Today object (parsed Today file)
pub type TaskList<'a> = BTreeMap<String, IdentifiedTask<'a>>;

// Passing a Today object as a parameter, this function should extract a TaskList from it and print
// the tasks one by one while formatting them properly.
pub fn print_tasklist_from(today: &Today)
{
    let list: &TaskList = &extracting::get_list_from(today);

    if list.is_empty()
    {
        println!("\tNo tasks around here just yet! •,•");
        return;
    }

    let mut text_lens: Vec<usize> = Vec::new();

    for task in list
    {
        text_lens.push(task.1.task.text.len());
    }

    for task in list
    {
        let id: &str = task.0.as_str();
        let checkbox = match task.1.task.done
        {
            true  => "[*]",
            false => "[ ]"
        };

        let text: &str = task.1.task.text.as_str();
        let label: String = format!("[{}]", task.1.owner_table);

        let space: usize = text_lens
                            .iter()
                            .max()
                            .unwrap_or(&40)
                            + 24 - text.len();
        
        println!("{id: <2} {checkbox} {text} {label:>space$}");
    }
}

// From a Today object, with also being fed the id for the task you are looking for from inside the
// Today object, this function changes the completion status of a task as you want, that being
// specified with the parameter choice.
pub fn set_done_to(today: &mut Today, task_id: &str, choice: bool) -> Result<bool, String>
{
    if task_id == "none"
    {
        return Ok(false);
    }
    
    let owner_table = if today.essential.contains_key(task_id)
    {
        &mut today.essential
    }
    else if today.optional.contains_key(task_id)
    {
        &mut today.optional
    }
    else
    {
        return Err(format!("Nor essential nor optional contain {}", task_id).to_string())
    };

    if let Some(task) = owner_table.get_mut(task_id)
    {
        if task.done == choice
        {
            let prefix = match choice
            {
                true  => String::new(),
                false => String::from("un")
            };

            warning::print_warning(format!("The task {task_id} is already {prefix}marked!").as_str());
        }

        task.done = choice;
        Ok(true)
    }
    else
    {
        Err(format!("Couldn't get {} from owner table", task_id))
    }
}
