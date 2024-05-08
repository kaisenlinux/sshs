pub mod searchable;
pub mod ssh;
pub mod ssh_config;
pub mod ui;

use anyhow::Result;
use clap::Parser;
use ui::{App, AppConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the SSH configuration file
    #[arg(
        short,
        long,
        num_args = 1..,
        default_values_t = [
            "/etc/ssh/ssh_config".to_string(),
            "~/.ssh/config".to_string(),
        ],
    )]
    config: Vec<String>,

    /// Shows ProxyCommand
    #[arg(long, default_value_t = false)]
    show_proxy_command: bool,

    /// Host search filter
    #[arg(short, long)]
    search: Option<String>,

    /// Sort hosts by hostname
    #[arg(long, default_value_t = true)]
    sort: bool,

    /// Handlebars template of the command to execute
    #[arg(short, long, default_value = "ssh \"{{{name}}}\"")]
    template: String,

    /// Exit after ending the SSH session
    #[arg(short, long, default_value_t = false)]
    exit: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut app = App::new(&AppConfig {
        config_paths: args.config,
        search_filter: args.search,
        sort_by_name: args.sort,
        show_proxy_command: args.show_proxy_command,
        command_template: args.template,
        exit_after_ssh: args.exit,
    })?;
    app.start()?;

    Ok(())
}
