mod disk;
mod ext4;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use disk::Disk;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList {
    state: ListState,
    items: Vec<ext4::structs::dir::Entry2>,
}
impl StatefulList {
    fn with_items(items: Vec<ext4::structs::dir::Entry2>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn top(&mut self) {
        self.state.select(Some(0));
    }
}

struct App {
    items: StatefulList,
    disk: Disk,
}
impl App {
    fn new(path: &str) -> App {
        let mut d = disk::Disk::new(path);
        App {
            items: StatefulList::with_items(d.read_dir(2)),
            disk: d,
        }
    }

    fn load(&mut self) {
        if let Some(x) = self.items.state.selected() {
            let entry = &self.items.items[x];
            if entry
                .file_type
                .intersects(ext4::flags::dir::FileType::Ext4FtDir)
            {
                self.items = StatefulList::with_items(self.disk.read_dir(entry.inode));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} file_name", args[0]);
        return Ok(());
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(&args[1]);
    run_app(&mut terminal, app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.items.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
                    KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => app.load(),
                    KeyCode::Left | KeyCode::Char('h') => app.items.top(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Ext4Impl")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.to_char()).style(Style::default().bg(Color::Black).fg(Color::White))
        })
        .collect();

    f.render_stateful_widget(
        List::new(items)
            .block(block)
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightGreen)),
        size,
        &mut app.items.state,
    );
}
