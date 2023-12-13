use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io;
use colored::Colorize;

pub fn navigate_strings(strings: &[String]) -> Option<Vec<String>> { //TODO: Fix the disappearing cursor bug
    let mut stdout = io::stdout();
    let mut current_index: usize = 0;
    let mut selected_indexes:Vec<usize> = Vec::new();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();
    let mut escape_pressed = false;

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();

        stdout.execute(MoveTo(0, 0)).unwrap();
        println!("Select files which you wish to be included in the commit message.");
        stdout.execute(MoveTo(0, 1)).unwrap();
        println!("Select {} when you are finished or {} to exit.\n", "Done".green(), "Escape".green());
        for (i, string) in strings.iter().enumerate() {
            let is_selected = selected_indexes.contains(&i);
            let is_current = i == current_index;
            let done_position = strings.len() - 1;
            
            stdout.execute(MoveTo(0, (i+3) as u16)).unwrap();
            if is_current && is_selected {
                println!("-> [{}] {}", "✓".green(), string);
            } else if i == done_position && is_current {
                println!("-> {}", string.green());
            } else if i == done_position {
                println!("   {}", string);
            } else if is_current {
                println!("-> [ ] {}", string);
            } else if is_selected {
                println!("   [{}] {}", "✓".green(), string);
            } else {
                println!("   []  {}", string);
            }
        }

        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Up => {
                    if current_index > 0 {
                        current_index -= 1;
                    } else {
                        current_index = strings.len() - 1;
                    }
                }
                KeyCode::Down => {
                    if current_index < strings.len() - 1 {
                        current_index += 1;
                    } else {
                        current_index = 0;
                    }
                }
                KeyCode::Esc  => {
                    escape_pressed = true;
                    break;
                }
                KeyCode::Enter => {

                     // Operation is done, return to the calling thread
                    if current_index == strings.len() - 1 {
                        break;
                    }

                    let pos = selected_indexes.iter().position(|&x| x == current_index);

                    match pos {
                        Some(index) => {
                            // If the current_index is found, remove it
                            selected_indexes.remove(index);
                        }
                        None => {
                            // If the current_index is not found, add it
                            selected_indexes.push(current_index);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    stdout.execute(MoveTo(0, 0)).unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();
    terminal::disable_raw_mode().unwrap();

    if selected_indexes.is_empty() || escape_pressed {
        None
    } else {
        let selected_strings: Vec<String> = selected_indexes
        .iter()
        .filter_map(|&index| strings.get(index).cloned())
        .collect();

        Some(selected_strings)
    }
}

