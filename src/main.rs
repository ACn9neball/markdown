use std::{fs::File, io::Read};

use clap::Parser;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
};

#[derive(Debug, Parser)]
struct CLI {
    #[arg(short, long)]
    path: String,
}

struct State {
    offset: usize,
}

impl State {
    fn up(&mut self) {
        self.offset += 1;
    }

    fn down(&mut self) {
        self.offset -= 1;
    }
}

fn main() -> color_eyre::Result<()> {
    let arg = CLI::parse();
    let mut state = State { offset: 0 };
    let mut file: File = match File::open(arg.path) {
        Ok(file) => file,
        Err(error) => {
            panic!("File not found! {}", error);
        }
    };

    let mut contents = String::new();
    let mut count: usize = 0;
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            for _ in contents.lines() {
                count += 1;
            }
        }
        Err(e) => panic!("Can't read file, {}", e),
    }

    println!("{}", &count);

    color_eyre::install()?;
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut contents, state.offset))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Char('k') | KeyCode::Up => {
                        if state.offset > 0 {
                            state.down();
                        }
                    }
                    KeyCode::Char('l') | KeyCode::Down => {
                        if (state.offset + 26) <= (count - 1) {
                            state.up();
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame, contents: &mut String, offset: usize) {
    let area = frame.area();
    let mut count = offset;

    let split = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Fill(1); 26])
        .split(area);

    while count < offset + 26 {
        let line: Vec<&str> = contents.lines().collect();
        if line[count].starts_with("###") {
            frame.render_widget(
                Paragraph::new(line[count].replacen("###", "", 1)).bold(),
                split[count - offset],
            );
        } else if line[count].starts_with("##") {
            frame.render_widget(
                Paragraph::new(line[count].replacen("##", "", 1))
                    .bold()
                    .centered(),
                split[count - offset],
            );
        } else if line[count].starts_with("#") {
            frame.render_widget(
                Paragraph::new(line[count].replacen("#", "", 1))
                    .bold()
                    .underlined()
                    .centered(),
                split[count - offset],
            );
        } else {
            frame.render_widget(Paragraph::new(line[count]), split[count - offset]);
        }
        count += 1;
    }
}
