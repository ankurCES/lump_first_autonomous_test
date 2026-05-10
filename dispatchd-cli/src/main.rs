use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use comfy_table::{Cell, CellAlignment, Table};
use dispatchd_core::{Job, Priority, Status, Store};
use std::io::IsTerminal;

#[derive(Parser)]
#[command(name = "dispatchd", about = "Tiny job queue CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new job
    Add {
        title: String,
        #[arg(long, default_value = "med")]
        priority: PriorityCli,
    },
    /// List jobs
    List {
        #[arg(long, default_value = "open")]
        status: StatusCli,
    },
    /// Cancel a job by id
    Cancel { id: u64 },
    /// Complete a job by id
    Complete { id: u64 },
}

#[derive(Clone, ValueEnum)]
enum PriorityCli {
    High,
    Med,
    Low,
}

impl From<PriorityCli> for Priority {
    fn from(p: PriorityCli) -> Self {
        match p {
            PriorityCli::High => Priority::High,
            PriorityCli::Med => Priority::Med,
            PriorityCli::Low => Priority::Low,
        }
    }
}

#[derive(Clone, ValueEnum)]
enum StatusCli {
    Open,
    Done,
    Cancelled,
    All,
}

impl StatusCli {
    fn to_filter(self) -> Option<Status> {
        match self {
            StatusCli::Open => Some(Status::Open),
            StatusCli::Done => Some(Status::Done),
            StatusCli::Cancelled => Some(Status::Cancelled),
            StatusCli::All => None,
        }
    }
}

fn colour_enabled() -> bool {
    std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal()
}

fn priority_coloured(p: &Priority) -> String {
    let label = format!("{p:?}");
    if !colour_enabled() {
        return label;
    }
    match p {
        Priority::High => label.red().to_string(),
        Priority::Med => label.yellow().to_string(),
        Priority::Low => label.green().to_string(),
    }
}

fn print_jobs_table(jobs: &[&Job]) {
    let mut table = Table::new();
    table.set_header(vec!["ID", "Title", "Priority", "Status"]);

    // Right-align the ID column (index 0)
    table
        .column_mut(0)
        .expect("ID column exists")
        .set_cell_alignment(CellAlignment::Right);

    for job in jobs {
        table.add_row(vec![
            Cell::new(job.id),
            Cell::new(&job.title),
            Cell::new(priority_coloured(&job.priority)),
            Cell::new(format!("{:?}", job.status)),
        ]);
    }

    println!("{table}");
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut store = Store::load().context("failed to load store")?;

    match cli.command {
        Commands::Add { title, priority } => {
            let id = store.add(title.clone(), priority.into());
            store.save().context("failed to save store")?;
            println!("Added job #{id}: {title}");
        }
        Commands::List { status } => {
            let jobs = store.list(status.to_filter());
            if jobs.is_empty() {
                println!("No jobs found.");
            } else {
                print_jobs_table(&jobs);
            }
        }
        Commands::Cancel { id } => {
            store
                .cancel(id)
                .with_context(|| format!("cannot cancel job #{id}"))?;
            store.save().context("failed to save store")?;
            println!("Cancelled job #{id}");
        }
        Commands::Complete { id } => {
            store
                .complete(id)
                .with_context(|| format!("cannot complete job #{id}"))?;
            store.save().context("failed to save store")?;
            println!("Completed job #{id}");
        }
    }

    Ok(())
}
