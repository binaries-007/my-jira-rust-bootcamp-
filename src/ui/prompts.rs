use std::io::{self, Write};

use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    let name;
    let description;

    println!("----------------------------");
    println!("Epic Name: ");
    name = get_user_input();
    println!("Epic Description: ");
    description = get_user_input();

    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    let name;
    let description;

    println!("----------------------------");
    println!("Story Name: ");
    name = get_user_input();

    println!("Story Description: ");
    description = get_user_input();

    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]: ");

    let input: &str = &get_user_input().to_lowercase();

    match input {
        "N" | "n" => false,
        "Y" | "y" => true,
        "" => true,
        _ => false,
    }
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    print!("Are you sure you want to delete this story? [Y/n]: ");
    io::stdout().flush().unwrap();

    let input: &str = &get_user_input().to_lowercase();

    match input {
        "N" | "n" => false,
        "Y" | "y" => true,
        "" => true,
        _ => false,
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED): ");

    match get_user_input().as_str() {
        user_input => match user_input.parse::<u32>() {
            Ok(num) => match num {
                1 => Some(Status::Open),
                2 => Some(Status::InProgress),
                3 => Some(Status::Resolved),
                4 => Some(Status::Closed),
                _ => None,
            },
            _ => None,
        },
    }
}
