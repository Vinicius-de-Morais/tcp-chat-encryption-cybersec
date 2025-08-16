use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub mod cipher;

/// Representa uma mensagem no chat
#[derive(Debug, Clone)]
struct Message {
    content: String,
    is_mine: bool,
}

enum InputEvent {
    UserInput(String),
    ServerMessage(String),
}

/// Função separada que escuta mensagens do servidor em uma thread
fn spawn_receiver_thread(stream: &TcpStream, tx: Sender<InputEvent>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    thread::spawn(move || {
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // conexão fechada
                Ok(_) => {
                    let _ = tx.send(InputEvent::ServerMessage(line.trim().to_string()));
                }
                Err(_) => break,
            }
        }
    });
}

/// Conecta ao servidor e retorna a conexão
fn setup_tcp_connection(tx: Sender<InputEvent>) -> TcpStream {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Erro ao conectar");
    spawn_receiver_thread(&stream, tx);
    stream
}

/// Envia mensagem para o servidor
fn send_message(stream: &mut TcpStream, msg: &str) {
    if let Err(e) = stream.write_all(msg.as_bytes()) {
        eprintln!("Erro ao enviar: {}", e);
    }
    if let Err(e) = stream.write_all(b"\n") {
        eprintln!("Erro ao enviar quebra de linha: {}", e);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Communication channel
    let (tx, rx): (Sender<InputEvent>, Receiver<InputEvent>) = mpsc::channel();

    let mut stream = setup_tcp_connection(tx.clone());

    // State
    let mut input = String::new();
    let mut messages: Vec<Message> = Vec::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(size);

            let text: Vec<ratatui::text::Line> = messages
                .iter()
                .map(|msg| {
                    if msg.is_mine {
                        ratatui::text::Line::from(vec![Span::styled(
                            format!("{:>width$}", msg.content, width = (size.width - 4) as usize),
                            Style::default().fg(Color::Green),
                        )])
                    } else {
                        ratatui::text::Line::from(vec![Span::styled(
                            msg.content.clone(),
                            Style::default().fg(Color::White),
                        )])
                    }
                })
                .collect();

            let msg_block = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Chat"))
                .alignment(Alignment::Left);

            let input_block = Paragraph::new(input.as_str()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Type a message"),
            );

            f.render_widget(msg_block, chunks[0]);
            f.render_widget(input_block, chunks[1]);
        })?;

        // Handle key events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => input.push(c),
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Enter => {
                            if !input.trim().is_empty() {
                                let msg = input.clone();
                                messages.push(Message {
                                    content: msg.clone(),
                                    is_mine: true,
                                });
                                send_message(&mut stream, &msg);
                                input.clear();
                            }
                        }
                        KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        // Handle messages received from the server
        while let Ok(event) = rx.try_recv() {
            if let InputEvent::ServerMessage(content) = event {
                messages.push(Message {
                    content,
                    is_mine: false,
                });
                if messages.len() > 100 {
                    messages.remove(0);
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
