use crate::{today::{self, Today}, base::Base, task::{TaskList, IdentifiedTask, UnidentifiedTask}};
use std::collections::BTreeMap;

pub fn get_list_from<'a>(today: &'a Today) -> TaskList<'a>
{
    let mut list: TaskList = BTreeMap::new();

    for (id, task) in today.essential.iter().chain(today.optional.iter())
    {
        let label = match today.essential.contains_key(id)
        {
            true  => "essential",
            false => "optional"
        };

        let identified_task = IdentifiedTask { task: &task, owner_table: label };
        list.insert(id.to_string(), identified_task);
    }

    list
}

pub fn convert_base_to_today(base: &Base, date: &str) -> Today
{
    let mut today = Today { date: date.to_string(), essential: BTreeMap::new(), optional: BTreeMap::new() };

    for (id, text) in base.essential.iter().chain(base.optional.iter())
    {
        let task = UnidentifiedTask { text: text.to_string(), done: false };

        let owner_table: &mut today::TableTemplate = match base.essential.contains_key(id)
        {
            true  => &mut today.essential,
            false => &mut today.optional
        };

        owner_table.insert(id.to_string(), task);
    }

    today
}
