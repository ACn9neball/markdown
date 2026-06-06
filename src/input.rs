const DB: &str = "markdown.db";

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_textarea::TextArea;
use rusqlite::Connection;

#[derive(Debug)]
struct Projects {
    id: i64,
    title: String,
    b_description: String,
    d_description: String,
    progress: String,
}

#[derive(Debug)]
struct Features {
    id: i64,
    feature: String,
    unique_id: i64,
}

#[derive(Debug)]
struct Languages {
    id: i64,
    language: String,
    unique_id: i64,
}

pub fn start() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    let result = run(&mut terminal);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut switch: usize = 0;
    let mut count = 0;
    let mut edt_title = TextArea::default();
    let mut edt_b = TextArea::default();
    let mut edt_d = TextArea::default();
    let mut edt_features = TextArea::default();
    let mut edt_languages = TextArea::default();
    let mut edt_progress = TextArea::default();
    let colors = vec![
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
        Color::Reset,
    ];

    loop {
        terminal.draw(|frame| {
            render(
                frame,
                &mut edt_title,
                &mut edt_b,
                &mut edt_d,
                &mut edt_features,
                &mut edt_languages,
                &mut edt_progress,
                count,
                switch,
                colors.clone(),
            )
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        let title_text: String = edt_title.lines().join("\n");
                        let basic_text: String = edt_b.lines().join("\n");
                        let detailed_text: String = edt_d.lines().join("\n");
                        let features_text: String = edt_features.lines().join("\n");
                        let languages_text: String = edt_languages.lines().join("\n");
                        let progress_text: String = edt_progress.lines().join("\n");
                        count = 0;

                        if !title_text.is_empty() {
                            count += 1;
                        }
                        if !basic_text.is_empty() {
                            count += 1;
                        }
                        if !detailed_text.is_empty() {
                            count += 1;
                        }
                        if !features_text.is_empty() {
                            count += 1;
                        }
                        if !languages_text.is_empty() {
                            count += 1;
                        }
                        if !progress_text.is_empty() {
                            count += 1;
                        }

                        if switch != 5 {
                            switch += 1;
                        } else {
                            switch = 0;
                        }
                        continue;
                    }
                    KeyCode::BackTab => {
                        let title_text: String = edt_title.lines().join("\n");
                        let basic_text: String = edt_b.lines().join("\n");
                        let detailed_text: String = edt_d.lines().join("\n");
                        let features_text: String = edt_features.lines().join("\n");
                        let languages_text: String = edt_languages.lines().join("\n");
                        let progress_text: String = edt_progress.lines().join("\n");
                        count = 0;

                        if !title_text.is_empty() {
                            count += 1;
                        }
                        if !basic_text.is_empty() {
                            count += 1;
                        }
                        if !detailed_text.is_empty() {
                            count += 1;
                        }
                        if !features_text.is_empty() {
                            count += 1;
                        }
                        if !languages_text.is_empty() {
                            count += 1;
                        }
                        if !progress_text.is_empty() {
                            count += 1;
                        }
                        if switch != 0 {
                            switch -= 1;
                        } else {
                            switch = 5;
                        }
                        continue;
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if count == 6 {
                            let title: String = edt_title.lines().join("\n");
                            let basic: String = edt_b.lines().join("\n");
                            let detailed: String = edt_d.lines().join("\n");
                            let features: String = edt_features.lines().join("\n");
                            let languages: String = edt_languages.lines().join("\n");
                            let progress: String = edt_progress.lines().join("\n");

                            let c = Connection::open(DB)?;
                            let project = Projects {
                                id: 0,
                                title: title,
                                b_description: basic,
                                d_description: detailed,
                                progress: progress,
                            };
                            let mut projects = c.prepare("SELECT * FROM projects")?;
                            let project_iter = projects.query_map([], |row| {
                                Ok(Projects {
                                    id: row.get(0)?,
                                    title: row.get(1)?,
                                    b_description: row.get(2)?,
                                    d_description: row.get(3)?,
                                    progress: row.get(4)?,
                                })
                            })?;
                            c.execute(
                                "INSERT INTO projects (title, bDescription, dDescription, progress) VALUES (?1, ?2, ?3, ?4)",
                                (&project.title, &project.b_description, &project.d_description, &project.progress),
                            )?;
                            let mut id: i64 = 0;
                            for pro in project_iter {
                                let p = pro.unwrap();
                                id = p.id;
                            }

                            for i in 0..features.lines().count() {
                                let mut feat = features.lines();
                                let feature = Features {
                                    id: 0,
                                    feature: feat.nth(i).expect("").to_string(),
                                    unique_id: id,
                                };
                                c.execute(
                                    "INSERT INTO features (feature, unique_id) VALUES (?1, ?2)",
                                    (&feature.feature, &feature.unique_id),
                                )?;
                            }

                            for i in 0..languages.lines().count() {
                                let mut lang = languages.lines();
                                let language = Languages {
                                    id: 0,
                                    language: lang.nth(i).expect("").to_string(),
                                    unique_id: id,
                                };
                                c.execute(
                                    "INSERT INTO languages (language, unique_id) VALUES (?1, ?2)",
                                    (&language.language, &language.unique_id),
                                )?;
                            }
                            break;
                        }
                    }
                    _ => match switch {
                        0 => {
                            edt_title.input(key);
                        }
                        1 => {
                            edt_b.input(key);
                        }
                        2 => {
                            edt_d.input(key);
                        }
                        3 => {
                            edt_features.input(key);
                        }
                        4 => {
                            edt_languages.input(key);
                        }
                        5 => {
                            edt_progress.input(key);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
    Ok(())
}

fn render(
    frame: &mut Frame,
    edt_title: &mut TextArea,
    edt_b: &mut TextArea,
    edt_d: &mut TextArea,
    edt_features: &mut TextArea,
    edt_languages: &mut TextArea,
    edt_progress: &mut TextArea,
    i: usize,
    switch: usize,
    colors: Vec<Color>,
) {
    let area = frame.area();

    let border = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(border.clone(), area);
    let inner_area = border.inner(area);

    let split = Layout::vertical([
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(4, 18),
        Constraint::Ratio(4, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
        Constraint::Ratio(1, 18),
    ]);
    let [
        top,
        title,
        input_title,
        basic_description,
        input_b_description,
        detailed_description,
        input_d_description,
        middle,
        progress,
        input_progress,
        _empty,
        bottom,
    ] = split.areas(inner_area);

    let top_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Ratio(1, 10)]);
    let [main, count] = top_spilt.areas(top);

    frame.render_widget(
        Paragraph::new("> SYSTEM: Enter project metadata below").alignment(Alignment::Left),
        main,
    );
    frame.render_widget(Paragraph::new("[1/100]").alignment(Alignment::Right), count);

    frame.render_widget(
        Paragraph::new("[1] Project Title").alignment(Alignment::Left),
        title,
    );
    edt_title.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .fg(color_switch(switch, colors.clone())[0]),
    );
    frame.render_widget(edt_title.widget(), input_title);

    frame.render_widget(
        Paragraph::new("[2] Basic Description").alignment(Alignment::Left),
        basic_description,
    );
    edt_b.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .fg(color_switch(switch, colors.clone())[1]),
    );
    frame.render_widget(edt_b.widget(), input_b_description);

    frame.render_widget(
        Paragraph::new("[3] Detailed Description").alignment(Alignment::Left),
        detailed_description,
    );
    edt_d.set_block(
        Block::default()
            .borders(Borders::ALL)
            .fg(color_switch(switch, colors.clone())[2]),
    );
    frame.render_widget(edt_d.widget(), input_d_description);

    let middle_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [features, languages] = middle_spilt.areas(middle);
    let feature_spilt = Layout::vertical([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)]);
    let [feature, input_features] = feature_spilt.areas(features);
    frame.render_widget(
        Paragraph::new("[4] Features: ").alignment(Alignment::Left),
        feature,
    );
    edt_features.set_block(
        Block::default()
            .borders(Borders::ALL)
            .fg(color_switch(switch, colors.clone())[3]),
    );
    frame.render_widget(edt_features.widget(), input_features);

    let language_spilt = Layout::vertical([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)]);
    let [language, input_languages] = language_spilt.areas(languages);

    frame.render_widget(
        Paragraph::new("[5] Languages:").alignment(Alignment::Left),
        language,
    );
    edt_languages.set_block(
        Block::default()
            .borders(Borders::ALL)
            .fg(color_switch(switch, colors.clone())[4]),
    );
    frame.render_widget(edt_languages.widget(), input_languages);

    frame.render_widget(
        Paragraph::new("[6] Progress:").alignment(Alignment::Left),
        progress,
    );
    edt_progress.set_block(
        Block::default()
            .borders(Borders::BOTTOM)
            .fg(color_switch(switch, colors.clone())[5]),
    );
    frame.render_widget(edt_progress.widget(), input_progress);

    let bottom_spilt = Layout::horizontal([Constraint::Fill(1), Constraint::Ratio(1, 10)]);
    let [main, save] = bottom_spilt.areas(bottom);
    frame.render_widget(
        Paragraph::new(format!("FIELDS: {}/6", i)).alignment(Alignment::Center),
        main,
    );
    frame.render_widget(
        Paragraph::new("CTRL+S TO SAVE ").alignment(Alignment::Right),
        save,
    );
}

fn color_switch(switch: usize, mut colors: Vec<Color>) -> Vec<Color> {
    for i in 0..colors.len() {
        if i == switch {
            colors[i] = Color::Blue
        } else {
            colors[i] = Color::Reset
        }
    }
    return colors;
}
