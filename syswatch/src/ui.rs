use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Gauge, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(frame.area());

    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU").borders(Borders::ALL))
        .ratio(app.cpu_usage as f64 / 100.0)
        .label(format!("{:.1}%", app.cpu_usage));
    frame.render_widget(cpu_gauge, areas[0]);

    let ram_gauge = Gauge::default()
        .block(Block::default().title("RAM").borders(Borders::ALL))
        .ratio(app.ram_used as f64 / app.ram_total as f64)
        .label(format!(
            "{:.1}%",
            app.ram_used as f64 / app.ram_total as f64
        ));
    frame.render_widget(ram_gauge, areas[1]);

    let rows: Vec<Row> = app
        .processes
        .iter()
        .map(|p| Row::new(vec![p.as_str()]))
        .collect();

    let table = Table::new(rows, [Constraint::Min(0)])
        .header(Row::new(vec!["Process"]))
        .block(Block::default().title("Processes").borders(Borders::ALL));
    frame.render_widget(table, areas[2]);
}
