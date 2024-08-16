use std::{path::PathBuf, rc::Rc};

use anyhow::Result;

use my_jira::{get_user_input, wait_for_key_press, JiraDatabase, Navigator};

fn main() -> Result<()> {
    let manifes_dir = env!("CARGO_MANIFEST_DIR");
    let json_db_path = PathBuf::from(manifes_dir).join("data").join("db.json");

    if !json_db_path.exists() {
        panic!("Unable to load JSON database");
    }
    let db = JiraDatabase::new(json_db_path.to_string_lossy().into());
    let mut navigator = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().unwrap();

        let page = navigator.get_current_page();

        if let None = page {
            std::process::exit(-1);
        }

        let page = page.unwrap();

        if let Err(error) = page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
        };

        let user_input = get_user_input();

        let action = page.handle_input(&user_input).unwrap();

        if let Some(action) = action {
            navigator.handle_action(action)?;
        }
    }
}
