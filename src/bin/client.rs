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
// Importa as cifras
use crate::ciphers::cesar::Cesar;
use crate::ciphers::monoalphabetic::Monoalphabetic;
use crate::ciphers::vigenere::Vigenere;
use crate::ciphers::playfair::cipher::Playfair;
use ciphers::Cipher;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub mod ciphers;

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
    // Índice da mensagem selecionada (para highlight e decriptação)
    let mut selected_msg_idx: usize = 0;
    // Estado para cifra selecionada
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum CipherType {
        Caesar,
        Monoalphabetic,
        Playfair,
        Vigenere,
    }
    let mut selected_cipher = CipherType::Caesar;
    let cipher_names = ["Caesar", "Monoalphabetic", "Playfair", "Vigenere"];
    let mut cipher_idx = 0;

    // Controle de texto separado para campo de mensagem e campo de chave
    let mut input = String::new();
    let mut key_input = String::new();
    let mut editing_key = false; // false = editando mensagem, true = editando chave
    let mut decrypt_mode = false; // true = aguardando chave para decriptar mensagem
    let mut decrypted_text: Option<String> = None;
    let mut decrypt_key_input = String::new(); // campo exclusivo para chave de decriptação

    loop {
    terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(size);

            use ratatui::widgets::{Borders, BorderType, Block};
            let mut text: Vec<ratatui::text::Line> = Vec::new();
            for (i, msg) in messages.iter().enumerate() {
                let is_selected = i == selected_msg_idx;
                let line_style = if msg.is_mine {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                };
                let line = msg.content.clone();
                // Se selecionada, aplica bloco visual
                if is_selected {
                    // Espaço extra acima
                    text.push(ratatui::text::Line::from(vec![Span::raw("")]));
                    // Bloco visual com borda arredondada e cor azul
                    let border_color = Color::Blue;
                    let block_title = if msg.is_mine {
                        "Você".to_string()
                    } else {
                        "Recebida | Ctrl+Enter para decriptar".to_string()
                    };
                    // Mensagem dentro do bloco
                    let mut inner_lines = vec![
                        ratatui::text::Line::from(vec![Span::styled(line.clone(), line_style.add_modifier(ratatui::style::Modifier::BOLD))]),
                    ];
                    // Tradução (decriptada) sempre embaixo da mensagem selecionada
                    if decrypted_text.is_some() && !msg.is_mine {
                        let dec = decrypted_text.as_ref().unwrap();
                        inner_lines.push(ratatui::text::Line::from(vec![Span::styled(
                            format!("→ {}", dec),
                            Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::ITALIC | ratatui::style::Modifier::BOLD)
                        )]));
                    }
                    // Renderiza bloco visual (simula bloco com linhas)
                    let width = (size.width - 8).max(20) as usize;
                    let border_top = format!("╭{:─<w$}╮ {}", "", block_title, w = width-2-block_title.len().min(width-2));
                    let border_bot = format!("╰{:─<w$}╯", "", w = width-2);
                    text.push(ratatui::text::Line::from(vec![Span::styled(border_top, Style::default().fg(border_color))]));
                    for l in &inner_lines {
                        let content = if msg.is_mine {
                            format!("{:>width$}", l.spans[0].content, width = width-4)
                        } else {
                            format!("{:<width$}", l.spans[0].content, width = width-4)
                        };
                        text.push(ratatui::text::Line::from(vec![
                            Span::styled("│ ", Style::default().fg(border_color)),
                            Span::styled(content, l.spans[0].style),
                            Span::styled(" │", Style::default().fg(border_color)),
                        ]));
                    }
                    text.push(ratatui::text::Line::from(vec![Span::styled(border_bot, Style::default().fg(border_color))]));
                    // Espaço extra abaixo
                    text.push(ratatui::text::Line::from(vec![Span::raw("")]));
                } else {
                    // Mensagem normal
                    let width = (size.width - 8).max(20) as usize;
                    let content = if msg.is_mine {
                        format!("{:>width$}", line, width = width)
                    } else {
                        format!("{:<width$}", line, width = width)
                    };
                    text.push(ratatui::text::Line::from(vec![Span::styled(content, line_style)]));
                }
            }
            let msg_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White))
                .title("Chat");
            let msg_paragraph = Paragraph::new(text)
                .block(msg_block)
                .alignment(Alignment::Left);

            // Campo de decriptação (aparece só no modo de decriptação)
            let decrypt_block = if decrypt_mode {
                let label = match selected_cipher {
                    CipherType::Caesar => "Chave para decriptar (número)",
                    CipherType::Monoalphabetic => "Chave para decriptar (26 letras)",
                    CipherType::Playfair => "Chave para decriptar (palavra)",
                    CipherType::Vigenere => "Chave para decriptar (palavra)",
                };
                Some(
                    Paragraph::new(decrypt_key_input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Yellow))
                                .title(label)
                        )
                        .style(Style::default().fg(Color::Yellow))
                )
            } else {
                None
            };


            let input_block_1 = Paragraph::new(input.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(if !editing_key { Style::default().fg(Color::Blue) } else { Style::default().fg(Color::White) })
                        .title("Mensagem (ENTER para enviar, TAB para chave)")
                )
                .style(Style::default().fg(Color::White));
            let input_block_2 = Paragraph::new(input.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White))
                        .title("Mensagem")
                )
                .style(Style::default().fg(Color::White));

            let key_label = match selected_cipher {
                CipherType::Caesar => "Chave (número)",
                CipherType::Monoalphabetic => "Chave (26 letras)",
                CipherType::Playfair => "Chave (palavra)",
                CipherType::Vigenere => "Chave (palavra)",
            };
            let key_block_1 = Paragraph::new(key_input.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(if editing_key { Style::default().fg(Color::Blue) } else { Style::default().fg(Color::White) })
                        .title(format!("{} [ENTER para confirmar, TAB para mensagem]", key_label))
                )
                .style(Style::default().fg(Color::White));
            let key_block_2 = Paragraph::new(key_input.as_str())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White))
                        .title(format!("{} (TAB para editar)", key_label))
                )
                .style(Style::default().fg(Color::White));

            // Rodapé para seleção de cifra
            let cipher_footer = Paragraph::new(format!(
                "Cifra: < {} > (← → para trocar)",
                cipher_names[cipher_idx]
            ))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Cifra"));

            f.render_widget(msg_paragraph.clone(), chunks[0]);
            if decrypt_mode {
                f.render_widget(msg_paragraph.clone(), chunks[0]);
                f.render_widget(input_block_2, chunks[1]);
                if let Some(decrypt_block) = &decrypt_block {
                    f.render_widget(decrypt_block, chunks[2]);
                }
            } else if editing_key {
                f.render_widget(key_block_1, chunks[1]);
                f.render_widget(input_block_2, chunks[2]);
                f.render_widget(msg_paragraph.clone(), chunks[0]);
            } else {
                f.render_widget(input_block_1, chunks[1]);
                f.render_widget(key_block_2, chunks[2]);
                f.render_widget(msg_paragraph, chunks[0]);
            }
            f.render_widget(cipher_footer, size);
        })?;

        // Handle key events
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if decrypt_mode {
                            match key.code {
                                KeyCode::Char(c) => decrypt_key_input.push(c),
                                KeyCode::Backspace => { decrypt_key_input.pop(); },
                                KeyCode::Enter => {
                                        if let Some(msg) = messages.get(selected_msg_idx) {
                                            let dec = match selected_cipher {
                                                CipherType::Caesar => {
                                                    let key: i8 = decrypt_key_input.trim().parse().unwrap_or(3);
                                                    let mut c = Cesar::new(key);
                                                    c.to_plaintext(&msg.content)
                                                },
                                                CipherType::Monoalphabetic => {
                                                    let mut c = Monoalphabetic::new(decrypt_key_input.trim().to_string());
                                                    c.to_plaintext(&msg.content)
                                                },
                                                CipherType::Playfair => {
                                                    let mut c = Playfair::new(decrypt_key_input.trim().to_string());
                                                    c.to_plaintext(&msg.content)
                                                },
                                                CipherType::Vigenere => {
                                                    let mut c = Vigenere::new(decrypt_key_input.trim().to_string());
                                                    c.to_plaintext(&msg.content)
                                                },
                                            };
                                            decrypted_text = Some(dec);
                                        }
                                        decrypt_mode = false;
                                        decrypt_key_input.clear();
                                },
                                KeyCode::Esc => {
                                    decrypt_mode = false;
                                    decrypt_key_input.clear();
                                },
                                _ => {}
                            }
                            continue;
                        }
                        if editing_key {
                            match key.code {
                                KeyCode::Char(c) => key_input.push(c),
                                KeyCode::Backspace => { key_input.pop(); },
                                KeyCode::Enter | KeyCode::Tab => {
                                    editing_key = false;
                                },
                                _ => {}
                            }
                            continue;
                        } else {
                            match key.code {
                                KeyCode::Enter => {
                                    if key.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) {
                                        if let Some(msg) = messages.get(selected_msg_idx) {
                                            if !msg.is_mine {
                                                decrypt_mode = true;
                                                key_input.clear();
                                            }
                                        }
                                    } else {
                                        // Envia mensagem normalmente
                                        if !input.trim().is_empty() {
                                            let msg = input.clone();
                                            let ciphered = match selected_cipher {
                                                CipherType::Caesar => {
                                                    let key: i8 = key_input.trim().parse().unwrap_or(3);
                                                    let mut c = Cesar::new(key);
                                                    c.to_ciphertext(&msg)
                                                },
                                                CipherType::Monoalphabetic => {
                                                    let mut c = Monoalphabetic::new(key_input.trim().to_string());
                                                    c.to_ciphertext(&msg)
                                                },
                                                CipherType::Playfair => {
                                                    let mut c = Playfair::new(key_input.trim().to_string());
                                                    c.to_ciphertext(&msg)
                                                },
                                                CipherType::Vigenere => {
                                                    let mut c = Vigenere::new(key_input.trim().to_string());
                                                    c.to_ciphertext(&msg)
                                                },
                                            };
                                            messages.push(Message {
                                                content: ciphered.clone(),
                                                is_mine: true,
                                            });
                                            send_message(&mut stream, &ciphered);
                                            input.clear();
                                            // Ao enviar mensagem, seleciona a última recebida
                                            selected_msg_idx = messages.len().saturating_sub(1);
                                            decrypted_text = None;
                                        }
                                    }
                                },
                                KeyCode::Char(c) => input.push(c),
                                KeyCode::Backspace => { input.pop(); },
                                KeyCode::Tab => {
                                    editing_key = true;
                                },
                                KeyCode::Up => {
                                    if selected_msg_idx > 0 {
                                        selected_msg_idx -= 1;
                                        decrypted_text = None;
                                    }
                                },
                                KeyCode::Down => {
                                    if selected_msg_idx + 1 < messages.len() {
                                        selected_msg_idx += 1;
                                        decrypted_text = None;
                                    }
                                },
                                KeyCode::Right => {
                                    cipher_idx = (cipher_idx + 1) % cipher_names.len();
                                    selected_cipher = match cipher_idx {
                                        0 => CipherType::Caesar,
                                        1 => CipherType::Monoalphabetic,
                                        2 => CipherType::Playfair,
                                        3 => CipherType::Vigenere,
                                        _ => CipherType::Caesar,
                                    };
                                    key_input.clear();
                                },
                                KeyCode::Left => {
                                    if cipher_idx > 0 {
                                        cipher_idx -= 1;
                                    } else {
                                        cipher_idx = cipher_names.len() - 1;
                                    }
                                    selected_cipher = match cipher_idx {
                                        0 => CipherType::Caesar,
                                        1 => CipherType::Monoalphabetic,
                                        2 => CipherType::Playfair,
                                        3 => CipherType::Vigenere,
                                        _ => CipherType::Caesar,
                                    };
                                    key_input.clear();
                                },
                                KeyCode::Esc => break,
                                _ => {}
                            }
                        }
                    }
                },
                Event::Mouse(_) => {
                    // Desabilitado: seleção de mensagem para decriptação via clique do mouse
                    // A seleção e entrada no modo de decriptação é feita apenas via Shift+Enter
                },
                _ => {}
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
                // Sempre seleciona a última mensagem recebida
                selected_msg_idx = messages.len().saturating_sub(1);
                decrypted_text = None;
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
