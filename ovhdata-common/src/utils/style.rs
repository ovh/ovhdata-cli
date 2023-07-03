use crossterm::style::{StyledContent, Stylize};
use descriptor::Describe;

pub fn underlined(str: &String) -> DescribedStyledContent {
    str.to_string().underlined().into()
}

pub fn bold_option(opt: &Option<String>) -> DescribedStyledContent {
    opt.clone().unwrap_or("".to_string()).bold().into()
}

pub fn color_status(state: &String) -> DescribedStyledContent {
    match state.as_str() {
        "PENDING" | "QUEUED" | "INITIALIZING" | "FINALIZING" => state.to_string().blue().into(),
        "FAILED" | "ERROR" => state.to_string().red().into(),
        "RUNNING" | "DONE" => state.to_string().green().into(),
        "TIMEOUT" | "INTERRUPTING" | "INTERRUPTED" => state.to_string().yellow().into(),
        _ => state.to_string().blue().into(),
    }
}

pub struct DescribedStyledContent {
    styled_content: StyledContent<String>,
}

impl From<StyledContent<String>> for DescribedStyledContent {
    fn from(styled_content: StyledContent<String>) -> Self {
        DescribedStyledContent { styled_content }
    }
}

impl Describe for DescribedStyledContent {
    fn to_field(&self, _: &str) -> String {
        self.styled_content.to_string()
    }
    fn default_headers() -> Vec<String> {
        Self::headers()
    }
    fn headers() -> Vec<String> {
        vec![]
    }
    fn header_name(_: &str) -> Option<String> {
        None
    }
}
