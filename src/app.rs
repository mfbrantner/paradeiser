use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::Text,
    widgets::Block,
    DefaultTerminal, Frame,
};
use std::sync::{Arc, Mutex};

use crate::pomodoro::state::PomodoroState;
use crate::{args::Args, timer_state::TimerState};

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    timer_state: Arc<Mutex<TimerState>>,
    args: Args,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            running: true,
            timer_state: Arc::new(Mutex::new(TimerState::Work("test".to_string()))),
            args,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        let a = self.args;
        let t = Arc::clone(&self.timer_state);

        std::thread::spawn(move || {
            Self::logic(a, t);
        });

        while self.running {
            self.poll_event()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn logic(args: Args, t: Arc<Mutex<TimerState>>) {
        let mut pomodoro_state_machine = PomodoroState::new(&args);
        loop {
            let pomodoro = pomodoro_state_machine.next();
            pomodoro.run(Arc::clone(&t));
        }
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        area
    }

    fn draw(&mut self, frame: &mut Frame) {

        let timer_state = self.timer_state.lock().unwrap();

        let (timer, timer_text, bg, fg) = match *timer_state {
            TimerState::Work(ref t) => (t, "Work", Color::Red, Color::White),
            TimerState::ShortBreak(ref t) => (t, "Short Break", Color::Yellow, Color::White),
            TimerState::LongBreak(ref t) => (t, "Long Break", Color::Green, Color::White),
        };

        let outer_block = Block::new().bg(bg);
        let text = Text::from(format!("{}: {}", timer_text, timer))
            .fg(fg)
            .bg(bg)
            .centered()
            .bold();
        let a = Self::center(
            frame.area(),
            Constraint::Length(text.width() as u16),
            Constraint::Length(1),
        );

        frame.render_widget(outer_block, frame.area());
        frame.render_widget(text, a);
    }

    fn poll_event(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            self.handle_crossterm_events()?
        }
        Ok(())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // quit application using q, Esc or Ctrl-C
            (_, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),

            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
