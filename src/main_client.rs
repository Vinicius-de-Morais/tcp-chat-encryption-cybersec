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

use tcp_chat::ciphers::rc4::cipher::Rc4;
use tcp_chat::ciphers::vigenere::Vigenere;
use tcp_chat::ciphers::Cipher;
use tcp_chat::ciphers::{cesar::Cesar, des::DES};
use tcp_chat::{ciphers::monoalphabetic::Monoalphabetic, protocol};
use tcp_chat::{ciphers::playfair::cipher::Playfair, protocol::CIPHERS};

use textwrap::wrap;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
    Terminal,
};

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

/// Envia mensagem para o servidor
fn send_message(stream: &mut TcpStream, msg: &Vec<u8>) {
    if let Err(e) = stream.write_all(msg) {
        eprintln!("Erro ao enviar: {}", e);
    }
    if let Err(e) = stream.write_all(b"\n") {
        eprintln!("Erro ao enviar quebra de linha: {}", e);
    }
}

fn decrypt(cipher: protocol::Cipher, key: String, encrypted: &Vec<u8>) -> Option<String> {
    let dec = match cipher {
        protocol::Cipher::Caesar => {
            let key: i8 = key.trim().parse().unwrap_or(3);
            let mut c = Cesar::new(key);
            c.to_plaintext(encrypted)
        }
        protocol::Cipher::MonoalphabeticSubstitution => {
            let mut c = Monoalphabetic::new(key.trim().to_string());
            c.to_plaintext(encrypted)
        }
        protocol::Cipher::Playfair => {
            let mut c = Playfair::new(key.trim().to_string());
            c.to_plaintext(encrypted)
        }
        protocol::Cipher::Vigenere => {
            let mut c = Vigenere::new(key.trim().to_string());
            c.to_plaintext(encrypted)
        }
        protocol::Cipher::Rc4 => {
            let mut c = Rc4::new(key.trim().to_string());
            c.to_plaintext(encrypted)
        }
        protocol::Cipher::Des => {
            let mut c = DES::new(&key.trim().as_bytes().to_vec());
            c.to_plaintext(encrypted)
        }
    };

    String::from_utf8(dec).ok()
}

fn encrypt(cipher: protocol::Cipher, key: String, plain: String) -> Option<Vec<u8>> {
    let key = key.trim().to_string();
    let plain = plain.as_bytes().to_vec();

    let ciphered = match cipher {
        protocol::Cipher::Caesar => {
            let key: i8 = key.parse().unwrap_or(3);
            let mut c = Cesar::new(key);
            c.to_ciphertext(&plain)
        }
        protocol::Cipher::MonoalphabeticSubstitution => {
            let mut c = Monoalphabetic::new(key);
            c.to_ciphertext(&plain)
        }
        protocol::Cipher::Playfair => {
            let mut c = Playfair::new(key);
            c.to_ciphertext(&plain)
        }
        protocol::Cipher::Vigenere => {
            let mut c = Vigenere::new(key);
            c.to_ciphertext(&plain)
        }
        protocol::Cipher::Rc4 => {
            let mut c = Rc4::new(key);
            c.to_ciphertext(&plain)
        }
        protocol::Cipher::Des => {
            let mut c = DES::new(&key.as_bytes().to_vec());
            c.to_ciphertext(&plain)
        }
    };

    Some(ciphered)
}

#[derive(PartialEq)]
enum InterfaceState {
    MessageInput,
    KeyInput,
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

    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Erro ao conectar");
    spawn_receiver_thread(&stream, tx.clone());

    // State
    let mut messages: Vec<Message> = Vec::new();
    // Índice da mensagem selecionada (para highlight e decriptação)
    let mut selected_msg_idx: usize = 0;

    let mut selected_cipher = protocol::Cipher::Caesar;
    let mut cipher_idx = 0;

    // Controle de texto separado para campo de mensagem e campo de chave
    let mut ui_state = InterfaceState::MessageInput;
    let mut input = String::new();
    let mut key_input = "".to_string();
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
                    Constraint::Length(1),
                ])
                .split(size);

            let CHUNK_MESSAGES_LIST = chunks[0];
            let CHUNK_MSG_INPUT = chunks[1];
            let CHUNK_KEY_INPUT = chunks[2];
            let CHUNK_CIPHER_INDICATOR = chunks[3];

            use ratatui::widgets::{Block, BorderType, Borders};

            // renderizar mensagens
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
                    let border_color = Color::Blue;
                    let block_title = if msg.is_mine {
                        "Você".to_string()
                    } else {
                        "Recebida | F2 para decriptar".to_string()
                    };

                    let width = (size.width - 4).max(20) as usize;
                    let content_width = (width - 4).max(1);

                    // Borda de cima
                    let border_top = format!(
                        "╭{:─<w$}╮ {}",
                        "",
                        block_title,
                        w = width - block_title.len().min(width - 2) - 5
                    );
                    text.push(ratatui::text::Line::from(vec![Span::styled(
                        border_top,
                        Style::default().fg(border_color),
                    )]));

                    // Quebra a mensagem original em um VETOR de linhas
                    let wrapped_message_lines = wrap(&line, content_width);

                    // FAZ UM LOOP sobre cada linha do vetor
                    for single_line in wrapped_message_lines {
                        // 3. Formata CADA LINHA INDIVIDUALMENTE
                        let content = if msg.is_mine {
                            format!("{:>width$}", single_line, width = content_width)
                        } else {
                            format!("{:<width$}", single_line, width = content_width)
                        };
                        // Adiciona a linha formatada ao `text`
                        text.push(ratatui::text::Line::from(vec![
                            Span::styled("│ ", Style::default().fg(border_color)),
                            Span::styled(
                                content,
                                line_style.add_modifier(ratatui::style::Modifier::BOLD),
                            ),
                            Span::styled(" │", Style::default().fg(border_color)),
                        ]));
                    }

                    // A mesma lógica de loop para o texto decriptado
                    if let Some(dec) = decrypted_text.as_ref() {
                        if !msg.is_mine {
                            let full_dec_text = format!("→ {}", dec);
                            let wrapped_dec_lines = wrap(&full_dec_text, content_width);

                            for single_dec_line in wrapped_dec_lines {
                                let content =
                                    format!("{:<width$}", single_dec_line, width = content_width);
                                text.push(ratatui::text::Line::from(vec![
                                    Span::styled("│ ", Style::default().fg(border_color)),
                                    Span::styled(
                                        content,
                                        Style::default().fg(Color::Yellow).add_modifier(
                                            ratatui::style::Modifier::ITALIC
                                                | ratatui::style::Modifier::BOLD,
                                        ),
                                    ),
                                    Span::styled(" │", Style::default().fg(border_color)),
                                ]));
                            }
                        }
                    }

                    // Borda de baixo
                    let border_bot = format!("╰{:─<w$}╯", "", w = width - 2);
                    text.push(ratatui::text::Line::from(vec![Span::styled(
                        border_bot,
                        Style::default().fg(border_color),
                    )]));
                } else {
                    // Mensagem normal
                    let width = (size.width - 8).max(20) as usize;
                    let content = if msg.is_mine {
                        format!("  {:>width$}  ", line, width = width)
                    } else {
                        format!("  {:<width$}  ", line, width = width)
                    };
                    text.push(ratatui::text::Line::from(vec![Span::styled(
                        content, line_style,
                    )]));
                }
            }

            let msg_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::White))
                .title("Mensagens");
            let msg_paragraph = Paragraph::new(text)
                .block(msg_block)
                .alignment(Alignment::Left);

            let key_label = match selected_cipher {
                protocol::Cipher::Caesar => "Chave (número)",
                protocol::Cipher::MonoalphabeticSubstitution => "Chave (26 letras)",
                protocol::Cipher::Playfair => "Chave (palavra)",
                protocol::Cipher::Vigenere => "Chave (palavra)",
                protocol::Cipher::Rc4 => "Chave (palavra)",
                protocol::Cipher::Des => "Chave (hex)",
            };

            // Rodapé para seleção de cifra
            let cipher_footer = Paragraph::new(format!(
                "Cifra: < {} > (← → para trocar)",
                CIPHERS[cipher_idx].to_string()
            ))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);

            f.render_widget(cipher_footer, CHUNK_CIPHER_INDICATOR);
            f.render_widget(msg_paragraph.clone(), CHUNK_MESSAGES_LIST);

            // Campo de decriptação (aparece só no modo de decriptação)
            if decrypt_mode {
                f.render_widget(msg_paragraph.clone(), CHUNK_MESSAGES_LIST);

                let label = match selected_cipher {
                    protocol::Cipher::Caesar => "Chave para decriptar (número)",
                    protocol::Cipher::MonoalphabeticSubstitution => {
                        "Chave para decriptar (26 letras)"
                    }
                    protocol::Cipher::Playfair => "Chave para decriptar (palavra)",
                    protocol::Cipher::Vigenere => "Chave para decriptar (palavra)",
                    protocol::Cipher::Rc4 => "Chave para decriptar (palavra)",
                    protocol::Cipher::Des => "Chave para decriptar (hex)",
                };

                f.render_widget(
                    Paragraph::new(decrypt_key_input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Yellow))
                                .title(label),
                        )
                        .style(Style::default().fg(Color::Yellow)),
                    CHUNK_KEY_INPUT,
                );
            } else {
                if ui_state == InterfaceState::KeyInput {
                    let key_block_1 = Paragraph::new(key_input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(if ui_state == InterfaceState::KeyInput {
                                    Style::default().fg(Color::Blue)
                                } else {
                                    Style::default().fg(Color::White)
                                })
                                .title(format!(
                                    "{} [ENTER para confirmar, TAB para mensagem]",
                                    key_label
                                )),
                        )
                        .style(Style::default().fg(Color::White));

                    let input_block_2 = Paragraph::new(input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::White))
                                .title("Mensagem"),
                        )
                        .style(Style::default().fg(Color::White));

                    f.render_widget(msg_paragraph.clone(), CHUNK_MESSAGES_LIST);
                    f.render_widget(key_block_1, CHUNK_MSG_INPUT);
                    f.render_widget(input_block_2, CHUNK_KEY_INPUT);
                } else {
                    let input_block_1 = Paragraph::new(input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(if ui_state == InterfaceState::MessageInput {
                                    Style::default().fg(Color::Blue)
                                } else {
                                    Style::default().fg(Color::White)
                                })
                                .title("Mensagem (ENTER para enviar, TAB para chave)"),
                        )
                        .style(Style::default().fg(Color::White));

                    let key_block_2 = Paragraph::new(key_input.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::White))
                                .title(format!("{} (TAB para editar)", key_label)),
                        )
                        .style(Style::default().fg(Color::White));

                    f.render_widget(msg_paragraph, CHUNK_MESSAGES_LIST);
                    f.render_widget(input_block_1, CHUNK_MSG_INPUT);
                    f.render_widget(key_block_2, CHUNK_KEY_INPUT);
                }
            }
        })?;

        // Handle key events
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if decrypt_mode {
                            match key.code {
                                KeyCode::Char(c) => decrypt_key_input.push(c),
                                KeyCode::Backspace => {
                                    decrypt_key_input.pop();
                                }
                                KeyCode::Enter => {
                                    if let Some(msg) = messages.get(selected_msg_idx) {
                                        let content_buffer = msg.content.as_bytes().to_vec();

                                        decrypted_text = decrypt(
                                            selected_cipher,
                                            key_input.clone(),
                                            &content_buffer,
                                        );
                                    }
                                    decrypt_mode = false;
                                    decrypt_key_input.clear();
                                }
                                KeyCode::Esc => {
                                    decrypt_mode = false;
                                    decrypt_key_input.clear();
                                }
                                _ => {}
                            }
                            continue;
                        }

                        // ...............

                        if ui_state == InterfaceState::KeyInput {
                            match key.code {
                                KeyCode::Char(c) => key_input.push(c),
                                KeyCode::Backspace => {
                                    key_input.pop();
                                }
                                KeyCode::Enter | KeyCode::Tab => {
                                    ui_state = InterfaceState::MessageInput;
                                }
                                _ => {}
                            }
                            continue;
                        } else {
                            match key.code {
                                KeyCode::F(2) => {
                                    if let Some(msg) = messages.get(selected_msg_idx) {
                                        if !msg.is_mine {
                                            decrypt_mode = true;
                                            decrypt_key_input.clear();
                                        }
                                    }
                                }

                                KeyCode::Enter => {
                                    if key
                                        .modifiers
                                        .contains(crossterm::event::KeyModifiers::SHIFT)
                                    {
                                        if let Some(msg) = messages.get(selected_msg_idx) {
                                            if !msg.is_mine {
                                                decrypt_mode = true;
                                                key_input.clear();
                                            }
                                        }
                                    } else {
                                        // Envia mensagem normalmente
                                        if !input.trim().is_empty() {
                                            let ciphered = encrypt(
                                                selected_cipher,
                                                key_input.clone(),
                                                input.clone(),
                                            )
                                            .unwrap();
                                            send_message(&mut stream, &ciphered);
                                            messages.push(Message {
                                                content: String::from_utf8(ciphered).unwrap(),
                                                is_mine: true,
                                            });
                                            input.clear();
                                            // Ao enviar mensagem, seleciona a última recebida
                                            selected_msg_idx = messages.len().saturating_sub(1);
                                            decrypted_text = None;
                                        }
                                    }
                                }
                                KeyCode::Char(c) => input.push(c),
                                KeyCode::Backspace => {
                                    input.pop();
                                }
                                KeyCode::Tab => {
                                    ui_state = InterfaceState::KeyInput;
                                }
                                KeyCode::Up => {
                                    if selected_msg_idx > 0 {
                                        selected_msg_idx -= 1;
                                        decrypted_text = None;
                                    }
                                }
                                KeyCode::Down => {
                                    if selected_msg_idx + 1 < messages.len() {
                                        selected_msg_idx += 1;
                                        decrypted_text = None;
                                    }
                                }
                                KeyCode::Left => {
                                    if cipher_idx > 0 {
                                        cipher_idx -= 1;
                                    } else {
                                        cipher_idx = CIPHERS.len() - 1;
                                    }
                                    selected_cipher = CIPHERS[cipher_idx];
                                    key_input.clear();
                                }
                                KeyCode::Right => {
                                    cipher_idx = (cipher_idx + 1) % CIPHERS.len();
                                    selected_cipher = CIPHERS[cipher_idx];
                                    key_input.clear();
                                }
                                KeyCode::Esc => break,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle messages received from the server
        while let Ok(event) = rx.try_recv() {
            if let InputEvent::ServerMessage(content) = event {
                if content.trim().len() < 1 {
                    continue;
                }

                messages.push(Message {
                    content: content.trim().to_string(),
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
