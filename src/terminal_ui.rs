use std::io::{self, Write};

use termion::{
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct TerminalUI {
    current_option: Option<String>,
}

pub struct OptionItem<T> {
    pub name: String,
    pub value: T,
}

impl TerminalUI {
    pub fn new() -> Self {
        Self {
            current_option: None,
        }
    }

    #[allow(dead_code)]
    pub fn ask_for_string_input(&self, question: &str) -> io::Result<String> {
        let mut prompt = String::from(question);
        prompt.push_str("\n=");

        let mut stdout = io::stdout();
        stdout.write(prompt.as_bytes())?;
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    pub fn ask_single_option<'a, T>(
        &'a mut self,
        question: &str,
        options: &'a [OptionItem<T>],
    ) -> io::Result<&OptionItem<T>> {
        self.current_option = Some(options[0].name.clone());
        let mut index = 0;

        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode()?;

        write!(stdout, "{}", termion::cursor::Hide)?;

        self.draw_single_option(&mut stdout, question, options)?;

        for char in stdin.keys() {
            match char.unwrap() {
                termion::event::Key::Char('q') => break,
                termion::event::Key::Char('j') | termion::event::Key::Down => {
                    if index < options.len() - 1 {
                        index += 1;
                    }
                }
                termion::event::Key::Char('k') | termion::event::Key::Up => {
                    if index > 0 {
                        index -= 1;
                    }
                }
                termion::event::Key::Char('\n') => break,
                _ => {}
            }

            self.current_option = Some(options[index].name.clone());

            self.draw_single_option(&mut stdout, question, options)?;
        }

        write!(stdout, "{}", termion::cursor::Show)?;

        Ok(&options[index])
    }

    fn draw_single_option<T>(
        &self,
        stdout: &mut RawTerminal<io::Stdout>,
        question: &str,
        options: &[OptionItem<T>],
    ) -> io::Result<()> {
        let mut output = String::new();
        output.push_str(question);
        output.push_str("\r\n");

        for option in options {
            if let Some(current_option) = &self.current_option {
                if current_option == &option.name {
                    output.push_str(&format!(
                        "{}",
                        termion::color::Fg(termion::color::LightGreen)
                    ));
                } else {
                    output.push_str(&format!("{}", termion::color::Fg(termion::color::Reset)));
                }
            }

            output.push_str(&format!(
                "{}\r\n{}",
                option.name,
                termion::color::Fg(termion::color::Reset)
            ));
        }

        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )?;

        write!(stdout, "{}\r\n", output)?;
        Ok(())
    }
}
