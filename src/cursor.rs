use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};
use colored::Colorize;

fn navigate_strings(strings: &[String]) -> Option<Vec<usize>>{
    let mut stdout = io::stdout();
    let mut current_index: usize = 0;
    let mut selected_indexes:Vec<usize> = Vec::new();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(Hide).unwrap();

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();
        for (i, string) in strings.iter().enumerate() {
            let is_selected = selected_indexes.contains(&i);
            let is_current = i == current_index;
            let done_position = strings.len() - 1;
            stdout.execute(MoveTo(0, i as u16)).unwrap();
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

    stdout.execute(Show).unwrap();
    terminal::disable_raw_mode().unwrap();

    if selected_indexes.is_empty() {
        None
    } else {
        Some(selected_indexes)
    }
}

