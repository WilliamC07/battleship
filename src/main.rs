#![allow(unused_variables)]
mod game;

use std::iter;
use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::{prelude::*, widgets::*};
use ratatui::crossterm::style::Stylize;
use ratatui::style::Styled;
use ratatui::text::ToSpan;
use crate::Action::Nothing;
use crate::game::game::{Game, GameState, StartMenu, StartMenuState};

fn main() {
    // Game state
    let mut game_info = Game::new();
    let mut terminal = ratatui::init();

    // Networking threads: server + client

    // One thread for drawing the screen and handling input
    run(game_info, terminal);

    ratatui::restore();
}

enum Action {
    NextState,
    Quit,
    Nothing
}

fn run(mut game_info: Game, mut terminal: ratatui::DefaultTerminal) {
    // Otherwise there is some text from running `cargo run`.
    terminal.clear().expect("Failed to do initial clear");

    loop {
        match game_info.get_state() {
            GameState::Start(start_menu) => {
                handle_start_state_draw(start_menu, &mut terminal);
                match handle_start_state_input(start_menu) {
                    Action::Quit => break,
                    _ => {}
                }
            }
            GameState::Playing(_) => {}
        }
    }
}

/// Terminal:
/// Battleship
/// Name: <Name>
/// Enter game:
/// Host Server | Join Existing: <IP>
fn handle_start_state_draw(start_menu: &mut StartMenu, terminal: &mut ratatui::DefaultTerminal) {
    terminal.draw(|frame: &mut Frame| {
        let cursor = Span::styled("_", Style::default().bg(Color::White));
        let screen = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                iter::repeat(Constraint::Percentage(25)).take(4)
            )
            .split(frame.area());

        /// Title
        frame.render_widget(Paragraph::new("Battleship"), screen[0]);

        /// Enter name
        let name_header = Span::styled("Name: ", Style::default().fg(Color::Green));
        let name_field = Span::styled(start_menu.get_name(), Style::default().fg(Color::White));
        let mut name_spans = vec![
            name_header,
            name_field
        ];
        if *start_menu.get_state() == StartMenuState::EnterName {
            name_spans.push(cursor.to_span());
        }
        let name_line: Line = Line::from(name_spans);
        frame.render_widget(Paragraph::new(name_line), screen[1]);

        /// Enter game
        let enter_game_header = Span::styled("Enter game:", Style::default());
        frame.render_widget(Paragraph::new(enter_game_header), screen[2]);

        let enter_game_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec!(Constraint::Percentage(50), Constraint::Percentage(50)))
            .split(screen[3]);
        let mut host_server_text = "Host Game".to_span();
        if *start_menu.get_state() == StartMenuState::StartServer {
            host_server_text = host_server_text.set_style(Style::default().bg(Color::Yellow));
        }
        frame.render_widget(Paragraph::new(host_server_text), enter_game_layout[0]);

        let enter_ip = Span::styled("Server IP: ", Style::default().fg(Color::Green));
        let ip_field = Span::styled(start_menu.get_server_ip(), Style::default().fg(Color::White));
        let mut connect_server_spans = vec![enter_ip, ip_field];
        if *start_menu.get_state() == StartMenuState::EnterServer {
            connect_server_spans.push(cursor.to_span())
        }
        let connect_server_line = Line::from(connect_server_spans);
        frame.render_widget(Paragraph::new(connect_server_line), enter_game_layout[1]);

    }).expect("Cannot draw start frame");
}

fn handle_start_state_input(start_menu: &mut StartMenu) -> Action {
    if event::poll(Duration::from_millis(100)).expect("Cannot read input") {
        if let CEvent::Key(key) = event::read().expect("Cannot read event") {
            match key.code {
                KeyCode::Tab => {
                    start_menu.transition_next_state();
                    return Nothing;
                },
                KeyCode::Char(c) => start_menu.record_key_stroke(c),
                KeyCode::Backspace => start_menu.record_backspace(),
                KeyCode::Enter if *start_menu.get_state() == StartMenuState::EnterName => {
                    start_menu.transition_next_state();
                    return Nothing;
                }
                KeyCode::Enter => {
                    // move to next state
                    // Do some validation
                    return Nothing;
                }
                KeyCode::Esc => {
                    return Action::Quit;
                }
                _ => {
                    return Nothing;
                }
            }
        }
    }
    Nothing
}
