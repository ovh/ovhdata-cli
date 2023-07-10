use std::collections::HashMap;
use std::fmt::Arguments;
use std::io;
use std::io::{Stderr, Stdout, Write};
use std::ops::{Add, Not};
use std::sync::RwLock;

use console::Term;
use crossterm::style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use descriptor::{object_describe, table_describe_to_string, Describe, Describer};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Password, Select};
use lazy_static::lazy_static;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use serde::Serialize;
use spinners::{Spinner, Spinners};

use ovhdata_common::model::di::common::EnsureSecret;
use ovhdata_common::model::di::common::Parameter;
use ovhdata_common::model::di::connector::{ConnectorParameter, ConnectorValidator};
use ovhdata_common::BUG;

use crate::config::{Context, Toggle};
use crate::options::NameValue;
use crate::utils::{Error, Result};
use crate::CLI_NAME;

pub const HELP_MAIN: &str = include_str!("../../../doc/main-help.md");
pub const HELP_LOGIN_HOW_TO: &str = include_str!("../../../doc/login-how-to.md");
pub const HELP_NO_AUTH_HOW_TO: &str = include_str!("../../../doc/no-auth-how-to.md");
pub const HELP_NO_SERVICE_NAME_HOW_TO: &str = include_str!("../../../doc/no-service-name-how-to.md");
pub const HELP_LOGIN_SUCCESS: &str = include_str!("../../../doc/login-success.md");
pub const HELP_COMPLETION_HOW_TO: &str = include_str!("../../../doc/completion-how-to.md");
pub const HELP_UPGRADE: &str = include_str!("../../../doc/upgrade-info.md");
pub const HELP_UPGRADE_MANDATORY: &str = include_str!("../../../doc/upgrade-mandatory.md");

#[cfg(target_os = "windows")]
pub const VALID: &str = "";
#[cfg(not(target_os = "windows"))]
pub const VALID: &str = "✔ ";
#[cfg(target_os = "windows")]
pub const INVALID: &str = "";
#[cfg(not(target_os = "windows"))]
pub const INVALID: &str = "✘ ";

lazy_static! {
    // Colored by default
    pub static ref NO_COLOR: RwLock<bool> = RwLock::new(false);
    // Spinner by default
    pub static ref NO_SPINNER: RwLock<bool> = RwLock::new(false);
}

pub struct Printer;

impl Printer {
    pub fn start_spinner(message: &str) -> Option<Spinner> {
        let no_spinner = NO_SPINNER.read().unwrap().to_owned();
        if no_spinner {
            None
        } else {
            Some(Spinner::new(Spinners::Pong, message.to_string()))
        }
    }

    pub fn stop_spinner(spinner: Option<Spinner>) {
        if let Some(mut spin) = spinner {
            spin.stop_with_newline();
        }
    }

    pub fn ask_select(prompt: &str, items: &[&str], default: usize) -> Result<usize> {
        let reply_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(items)
            .default(default)
            .report(false)
            .interact()
            .map_err(|_| Error::UserInput)?;

        Ok(reply_index)
    }

    pub fn ask_select_table<T: Describe>(data: &[T], default_selection: Option<String>) -> Result<&T> {
        let table = table_describe_to_string(data).unwrap();
        let mut table_entries: Vec<_> = table.split('\n').collect();

        let prompt = format!("{}\n", table_entries.first().unwrap());
        table_entries.remove(0);

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&table_entries)
            .default(0)
            .report(false)
            .with_initial_text(default_selection.unwrap_or_default())
            .interact_on(&Term::stderr())?;

        if let Some(selected) = data.get(selection) {
            Ok(selected)
        } else {
            Err(Error::custom("Value not selected"))
        }
    }

    pub fn ask_connector_parameters(
        input: &[NameValue],
        api: Option<&Vec<Parameter>>,
        connector_parameters: &[ConnectorParameter],
    ) -> Result<Vec<Parameter>> {
        let input_parameters: HashMap<String, Parameter> = input
            .iter()
            .map(|parameter| (parameter.clone().name, parameter.clone().into()))
            .collect::<HashMap<_, _>>();

        let api_parameters: HashMap<String, Parameter> = if let Some(parameters) = api {
            parameters
                .iter()
                .map(|api_param| (api_param.clone().name, api_param.clone()))
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        };

        let mut parameters: Vec<Parameter> = Vec::new();

        // Non interactive mode
        // input_parameter have always the priority
        if !input_parameters.is_empty() {
            let params = connector_parameters
                .iter()
                .filter(|connector_parameter| input_parameters.contains_key(&connector_parameter.name))
                .map(|connector_parameter| {
                    // Adds the secret field to the parameter
                    // used internally for never display secret
                    let mut param = input_parameters.get(&connector_parameter.name).unwrap().to_owned();

                    if connector_parameter.type_name == "secret" {
                        param.secret = true;
                    }
                    param
                })
                .collect();
            return Ok(params);
        }

        // Interactive mode (input_parameter empty)
        // Can be use for create, update or delete a field for a given connector_parameters
        //  - Create: no api_parameter (not existing remotely, default from connector_parameters)
        //  - Update: api_parameter (value set as default)
        //  - Delete: api_parameter (value set as default) with empty value
        for connector_parameter in connector_parameters.iter() {
            let api_param = api_parameters.get(&connector_parameter.name);

            // Update case -> api as default
            let current_value = api_param.map(|param| param.value.clone());

            let param = Printer::ask_parameter(connector_parameter, current_value).unwrap();

            if param.is_none() {
                continue;
            }
            parameters.push(param.unwrap());
        }
        Ok(parameters)
    }

    fn get_validator_help(type_name: &str, option_validator: &Option<ConnectorValidator>) -> String {
        match option_validator {
            Some(validator) => match type_name {
                "string" => "None (string by default)".to_string(),
                "secret" => "None (string by default)".to_string(),
                "boolean" => "(true, True, false, False)".to_string(),
                "int" => {
                    if validator.min == validator.max {
                        "None (any integer without spaces)".to_string()
                    } else {
                        format!("Min value={} , max value={}", validator.min, validator.max)
                    }
                }
                _ => "None (string by default)".to_string(),
            },
            None => "None".to_string(),
        }
    }

    fn ask_parameter(connector_parameter: &ConnectorParameter, current_value: Option<String>) -> Result<Option<Parameter>> {
        let prompt = format!(
            "{}\n\t{} {}\n\t{} {}\n\t{} {}\n\t{} {}\n\t{} {}\n{}",
            "Enter parameter".blue(),
            "\u{251C} Name:".blue(),
            connector_parameter.name.clone(),
            "\u{251C} Type:".blue(),
            connector_parameter.type_name.clone(),
            "\u{251C} Validator:".blue(),
            Printer::get_validator_help(&connector_parameter.type_name, &connector_parameter.validator),
            "\u{251C} Description:".blue(),
            connector_parameter.description.clone(),
            "\u{2514} Mandatory:".blue(),
            connector_parameter.mandatory,
            connector_parameter.name.clone()
        );

        let default_value = if connector_parameter.mandatory && connector_parameter.default.is_some() {
            connector_parameter.clone().default
        } else {
            None
        };

        let mut parameter_secret = false;
        let parameter_value = match connector_parameter.type_name.clone().as_str() {
            "string" => {
                let value = Printer::ask_input_string(&prompt, current_value.clone(), connector_parameter.mandatory.not(), default_value);
                if value.is_some() {
                    value
                } else if !connector_parameter.mandatory && value.is_none() && current_value.is_some() {
                    // current value / none from keyboard and not mandatory, return empty string (delete it)
                    Some("".to_string())
                } else {
                    None
                }
            }
            "secret" => {
                parameter_secret = true;
                Printer::ask_password(&prompt, connector_parameter.mandatory.not())
            }
            "int" => {
                let validator = connector_parameter.validator.clone().unwrap();
                let value = Printer::ask_input_integer(
                    &prompt,
                    current_value.clone(),
                    connector_parameter.mandatory.not(),
                    &validator,
                    default_value,
                );
                if value.is_some() {
                    value
                } else if !connector_parameter.mandatory && value.is_none() && current_value.is_some() {
                    // current value / none from keyboard and not mandatory, return empty string (delete it)
                    Some("".to_string())
                } else {
                    None
                }
            }
            "boolean" => {
                let value: bool = if let Some(c_value) = current_value {
                    c_value.to_lowercase().parse().unwrap_or_default()
                } else if let Some(d_value) = default_value {
                    d_value.to_lowercase().parse().unwrap_or_default()
                } else {
                    false
                };

                let value = Printer::ask_input_boolean(&prompt, value);
                Some(value.unwrap().to_string())
            }
            _ => {
                return Err(Error::Custom(format!("Unsupported parameter type {}!", connector_parameter.type_name)));
            }
        };

        // Convert the value (string) into parameter
        if let Some(value) = parameter_value {
            let param = Parameter {
                name: connector_parameter.name.clone(),
                value,
                secret: parameter_secret,
            };
            Ok(Option::from(param))
        } else {
            Ok(None)
        }
    }

    pub fn ask_input_string(prompt: &str, initial_text: Option<String>, allow_empty: bool, default: Option<String>) -> Option<String> {
        let color_binding = ColorfulTheme::default();
        let mut input_binding = Input::with_theme(&color_binding);

        input_binding.with_prompt(prompt).report(false).allow_empty(allow_empty);

        if let Some(default_input) = default {
            input_binding.default(default_input);
        }

        if let Some(text) = initial_text {
            input_binding.with_initial_text(text);
        }

        input_binding.interact_text().map(|s: String| s.is_empty().not().then_some(s)).unwrap()
    }

    fn ask_input_integer(
        prompt: &str,
        initial_text: Option<String>,
        allow_empty: bool,
        validator: &ConnectorValidator,
        default: Option<String>,
    ) -> Option<String> {
        let color_binding = ColorfulTheme::default();
        let mut input_binding = Input::with_theme(&color_binding);

        input_binding.with_prompt(prompt).report(false).allow_empty(allow_empty).validate_with({
            move |input: &String| -> std::result::Result<(), String> {
                let test = input.parse::<i64>();
                match test {
                    Ok(value) => {
                        if value < validator.min {
                            Err(format!("The value can not be lower than {}", validator.min))
                        } else if value > validator.max {
                            Err(format!("The value can not be upper than {}", validator.max))
                        } else {
                            Ok(())
                        }
                    }
                    Err(_) => Err("Invalid value, this is not an integer".to_string()),
                }
            }
        });

        if let Some(default_input) = default {
            input_binding.default(default_input);
        }

        if let Some(text) = initial_text {
            input_binding.with_initial_text(text);
        }

        input_binding.interact_text().map(|s: String| s.is_empty().not().then_some(s)).unwrap()
    }

    pub fn ask_input_boolean(prompt: &str, default: bool) -> Result<bool> {
        match Printer::ask_select(prompt, &["False", "True"], default as usize).unwrap() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::UserInput), // Unreachable
        }
    }

    pub fn ask_input(prompt: &str, initial_text: Option<&str>) -> Result<String> {
        let color_binding = ColorfulTheme::default();
        let mut input_binding = Input::with_theme(&color_binding);

        input_binding.with_prompt(prompt);

        if let Some(text) = initial_text {
            input_binding.with_initial_text(text);
        }
        input_binding.interact().map_err(|_| Error::UserInput)
    }

    pub fn confirm(message: &str) -> Result<bool> {
        if !Confirm::with_theme(&ColorfulTheme::default()).with_prompt(message).interact()? {
            return Err(Error::custom("Operation cancelled by user"));
        }
        Ok(true)
    }

    pub fn ask_password(prompt: &str, allow_empty: bool) -> Option<String> {
        let color_binding = ColorfulTheme::default();
        let mut password_binding = Password::with_theme(&color_binding);

        password_binding
            .with_prompt(prompt)
            .allow_empty_password(allow_empty)
            .report(false)
            .interact()
            .map(|s: String| s.is_empty().not().then_some(s))
            .unwrap()
    }

    pub fn println_success(write: &mut dyn Write, msg: &str) {
        writeln!(write, "{}{}", VALID.dark_green(), msg.bold()).expect("can't write on stdout");
    }

    pub fn eprintln_fail(msg: &str) {
        writeln!(stderr(), "{}{}", INVALID.dark_red(), msg.dark_red().bold()).expect("can't write on stderr");
    }

    pub fn print_object<T>(data: &T, output: &Output) -> Result<()>
    where
        T: Serialize + Describe + EnsureSecret<T>,
    {
        let output_data = data.hide_secrets();

        match output {
            Output::Table(headers) => {
                Describer::describe_list_with_header(std::slice::from_ref(&output_data), headers, &mut stdout(), descriptor::Context::default())?;
                Ok(())
            }
            Output::Json => Self::print_json(&output_data),
            Output::Yaml => Self::print_yaml(&output_data),
            Output::Description => {
                object_describe(&output_data, &mut stdout())?;
                Ok(())
            }
        }
    }

    pub fn print_list<T>(data: &[T], output: &Output) -> Result<()>
    where
        T: Serialize + Describe,
    {
        match output {
            Output::Table(headers) => {
                if !data.is_empty() {
                    Describer::describe_list_with_header(data, headers.as_slice(), &mut stdout(), descriptor::Context::default())?;
                }
            }
            Output::Json => Self::print_json(&data)?,
            Output::Yaml => Self::print_yaml(&data)?,
            Output::Description => {
                for obj in data.iter() {
                    object_describe(obj, &mut stdout())?;
                    println!();
                }
            }
        }
        Ok(())
    }

    pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
        serde_json::to_writer(io::stdout(), &data).map_err(Error::custom)?;
        println!();

        Ok(())
    }

    pub fn print_yaml<T: Serialize>(data: &T) -> Result<()> {
        serde_yaml::to_writer(io::stdout(), &data).map_err(Error::custom)?;
        Ok(())
    }

    pub fn print_command(command: &str) {
        println!();
        writeln!(stdout(), "Running the following command:").unwrap();
        writeln!(stdout(), "> {} {}", CLI_NAME.bold(), command.bold()).unwrap();
        writeln!(
            stdout(),
            "(consider adding the --no-spinner, --no-color and -f options to use this command in a script)"
        )
        .unwrap();
    }

    pub fn print_help(markdown: &str, toggle: Toggle) {
        // Check if this help is toggled
        if Context::get().is_toggle(toggle) {
            return;
        }

        // Generate and print help
        let help = Self::gen_help(markdown);
        print!("{}", help);
    }

    // Generate help from markdown
    pub fn gen_help(markdown: &str) -> String {
        let md_style = MarkdownStyle {
            h1: None,
            h2: Some(Color::Blue),
            emphasis: Some(Color::Cyan),
            foreground: None,
            background: None,
            padding_top: 0,
            padding_bottom: 0,
        };
        Self::render_markdown(markdown, &md_style)
    }

    // Render output from markdown
    pub fn render_markdown(markdown: &str, md_style: &MarkdownStyle) -> String {
        let mut output = Vec::new();

        if let Some(bg_color) = md_style.background {
            write!(&mut output, "{}{}", SetBackgroundColor(bg_color), "\n".repeat(md_style.padding_top)).unwrap();
        }
        let foreground: Color = match md_style.foreground {
            Some(fg_color) => {
                write!(&mut output, "{}", SetForegroundColor(fg_color)).unwrap();
                fg_color
            }
            None => Color::Reset,
        };

        // Render markdown
        let parser = Parser::new(markdown);
        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Heading(HeadingLevel::H1, ..) => {
                        write!(&mut output, "{}", Attribute::Bold).unwrap();
                        if let Some(color) = md_style.h1 {
                            write!(&mut output, "{}", SetForegroundColor(color)).unwrap();
                        }
                    }
                    Tag::Heading(HeadingLevel::H2, ..) => {
                        if let Some(color) = md_style.h2 {
                            write!(&mut output, "{}", SetForegroundColor(color)).unwrap();
                        }
                    }
                    Tag::Item => {
                        write!(&mut output, "  • ",).unwrap();
                    }
                    Tag::Strong => {
                        write!(&mut output, "{}", Attribute::Bold).unwrap();
                    }
                    Tag::Emphasis => {
                        if let Some(color) = md_style.emphasis {
                            write!(&mut output, "{}", SetForegroundColor(color)).unwrap();
                        }
                    }
                    Tag::Link(_, url, _) => {
                        write!(
                            &mut output,
                            "{}{}{}",
                            SetAttribute(Attribute::Underlined),
                            url,
                            SetAttribute(Attribute::NoUnderline)
                        )
                        .unwrap();
                    }
                    _ => {}
                },
                Event::End(tag) => match tag {
                    Tag::Paragraph => {
                        write!(&mut output, "\n\n").unwrap();
                    }
                    Tag::Heading(HeadingLevel::H1, ..) => {
                        if md_style.h1.is_some() {
                            write!(&mut output, "{}", SetForegroundColor(foreground)).unwrap();
                        }
                        write!(&mut output, "{}\n\n", Attribute::NormalIntensity).unwrap();
                    }
                    Tag::Heading(HeadingLevel::H2, ..) => {
                        if md_style.h2.is_some() {
                            writeln!(&mut output, "{}", SetForegroundColor(foreground)).unwrap();
                        }
                    }
                    Tag::CodeBlock(_) => {
                        writeln!(&mut output).unwrap();
                    }
                    Tag::List(_) => {
                        writeln!(&mut output).unwrap();
                    }
                    Tag::Item => {
                        writeln!(&mut output).unwrap();
                    }
                    Tag::Strong => {
                        write!(&mut output, "{}", Attribute::NormalIntensity).unwrap();
                    }
                    Tag::Emphasis => {
                        if md_style.emphasis.is_some() {
                            write!(&mut output, "{}", SetForegroundColor(foreground)).unwrap();
                        }
                    }
                    _ => {}
                },
                Event::Text(text) => {
                    write!(&mut output, "{}", text).unwrap();
                }
                _ => {}
            }
        }

        if md_style.background.is_some() {
            write!(
                &mut output,
                "{}{}",
                "\n".repeat(md_style.padding_bottom),
                SetBackgroundColor(Color::Reset)
            )
            .unwrap();
        }
        if md_style.foreground.is_some() {
            write!(&mut output, "{}", SetForegroundColor(Color::Reset)).unwrap();
        }

        String::from_utf8(output)
            .map(|rendered| match md_style.background {
                // When background is set, prepend any line break with "Erase in Line" ANSI code
                // for applying bg color on the whole line.
                // Finalize with a last "Erase in Line" for restoring the default bg color on next line.
                Some(_) => rendered
                    .replace('\n', &format!("{}\n", Clear(ClearType::UntilNewLine)))
                    .add(&format!("{}", Clear(ClearType::UntilNewLine))),
                None => rendered,
            })
            .unwrap()
    }
}

pub struct MarkdownStyle {
    h1: Option<Color>,
    h2: Option<Color>,
    emphasis: Option<Color>,
    foreground: Option<Color>,
    background: Option<Color>,
    padding_top: usize,
    padding_bottom: usize,
}

#[derive(Clone, PartialEq)]
pub enum Output {
    Table(Vec<String>),
    Json,
    Yaml,
    Description,
}

impl Output {
    pub fn default_table() -> Output {
        Output::Table(Vec::new())
    }
}

pub struct PrinterStdout(Stdout, bool);

impl Write for PrinterStdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        let mut buffer = Vec::with_capacity(128);
        buffer.write_fmt(fmt)?;
        if !self.1 {
            self.0.write_all(&strip_ansi_escapes::strip(buffer)?)
        } else {
            self.0.write_all(&buffer)
        }
    }
}

pub fn stdout() -> PrinterStdout {
    if *NO_COLOR.read().expect(BUG) {
        PrinterStdout(io::stdout(), false)
    } else {
        PrinterStdout(io::stdout(), true)
    }
}

pub struct PrinterStdErr(Stderr, bool);

impl Write for PrinterStdErr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> io::Result<()> {
        let mut buffer = Vec::with_capacity(128);
        buffer.write_fmt(fmt)?;
        if !self.1 {
            self.0.write_all(&strip_ansi_escapes::strip(buffer)?)
        } else {
            self.0.write_all(&buffer)
        }
    }
}

pub fn stderr() -> PrinterStdErr {
    if *NO_COLOR.read().expect(BUG) {
        PrinterStdErr(io::stderr(), false)
    } else {
        PrinterStdErr(io::stderr(), true)
    }
}
