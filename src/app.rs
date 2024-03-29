use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author = "Guy Edwards <guyfedwards@gmail.com>",
    version = "1.2",
    about = "Simple pomodoro timer"
)]
pub struct App {
    /// This command will be ran on the remote nodes
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// Generate Bash completion to get bash shell completion to work you can add
    /// `eval "$(omo completion bash|elvish|fish|powershell|zsh)"` to your shell
    /// start up script.
    #[clap(verbatim_doc_comment)]
    Completion { shell: clap_complete::Shell },

    /// Get remaining time
    Get {
        #[arg(
            short,
            long,
            value_name = "MESSAGE",
            default_value = "Omo timer",
            help = "trigger system notification if 20 mins has passed when called"
        )]
        notify: String,
    },

    /// Reset timer to 20 mins
    Reset {
        /// Reset the timer to a specified amount of minutes
       #[arg(short, long = "min")]
        minutes: Option<i64>,
    },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Completion { .. } => write!(f, "Completion",),
            Self::Get { .. } => write!(f, "Get",),
            Self::Reset { .. } => write!(f, "Reset",),
        }
    }
}
