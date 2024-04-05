use std::collections::BTreeMap;
use crate::{extracting, today::Today, files};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct UnidentifiedTask
{
    pub text: String,
    pub done: bool
}

pub struct IdentifiedTask<'a>
{
    pub task: &'a UnidentifiedTask,
    pub owner_table: &'a str
}

pub type TaskList<'a> = BTreeMap<String, IdentifiedTask<'a>>;

pub fn print_tasklist_from(today: &Today)
{
    let list: &TaskList = &extracting::get_list_from(today);

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

pub fn set_done_to(today: &mut Today, task_id: &str, choice: bool) -> Result<String, String>
{
    if task_id == "none"
    {
        return Ok(String::new())
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
        task.done = choice;
        let new_contents: String = files::get_new_contents(today)?;
        Ok(new_contents)
    }
    else
    {
        Err(format!("Couldn't get {} from owner table", task_id))
    }
}
