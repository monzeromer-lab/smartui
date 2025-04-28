use clap::{Parser, Subcommand};
use commands::explain::GenerateContent;
use std::path::PathBuf;

mod gemini;
mod commands;
mod error;

use error::{Result, GmError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Explain a terminal command
    Explain {
        /// The command to explain
        command: String,
    },
    /// Summarize text from a file or stdin
    Summarize {
        /// Path to the file to summarize
        file: Option<PathBuf>,
        /// Read from stdin instead of a file
        #[arg(long)]
        stdin: bool,
    },
    /// Translate text to a target language
    Translate {
        /// Text to translate
        text: String,
        /// Target language code (e.g., fr, es)
        #[arg(long)]
        to: String,
    },
    /// Define a word or phrase
    Define {
        /// Word or phrase to define
        term: String,
        /// Optional context for the definition
        #[arg(long)]
        context: Option<String>,
    },
    /// Generate ideas based on keywords
    Brainstorm {
        /// Keywords or description
        topic: String,
    },
    /// Rephrase text in a different style
    Rephrase {
        /// Text to rephrase
        text: String,
        /// Style to use (e.g., formal, informal, technical)
        #[arg(long)]
        style: Option<String>,
    },
    /// Fix typos in text
    FixTypos {
        /// Text to correct
        text: String,
    },
    /// Generate a shell alias
    GenerateAlias {
        /// Description of the task
        description: String,
    },
    /// Analyze sentiment of text
    AnalyzeSentiment {
        /// Text to analyze
        text: String,
        /// Read from stdin instead of arguments
        #[arg(long)]
        stdin: bool,
    },
    /// Explain code from a file
    ExplainCode {
        /// Path to the code file
        file: PathBuf,
        /// Programming language hint
        #[arg(long)]
        language: Option<String>,
    },
    /// Explain an error message
    ExplainError {
        /// Error message to explain
        error: String,
        /// Read from stdin instead of arguments
        #[arg(long)]
        stdin: bool,
    },
    /// Generate a regular expression
    GenerateRegex {
        /// Description of the pattern
        pattern: String,
    },
    /// Convert data between formats
    ConvertFormat {
        /// Path to the input file
        file: Option<PathBuf>,
        /// Read from stdin instead of a file
        #[arg(long)]
        stdin: bool,
        /// Source format
        #[arg(long)]
        from: String,
        /// Target format
        #[arg(long)]
        to: String,
    },
    /// Explain a concept
    ExplainConcept {
        /// Concept to explain
        concept: String,
        /// Field or context
        #[arg(long)]
        field: Option<String>,
    },
    /// Find commands for a task
    FindCommand {
        /// Description of the task
        task: String,
    },
    /// Generate boilerplate code
    GenerateBoilerplate {
        /// Description of the boilerplate
        description: String,
    },
    /// Review text
    ReviewText {
        /// Text to review
        text: String,
        /// Read from stdin instead of arguments
        #[arg(long)]
        stdin: bool,
        /// Review criteria
        #[arg(long)]
        criteria: Option<String>,
    },
    /// Suggest names
    SuggestName {
        /// Description of what needs naming
        description: String,
    },
    /// Explain an algorithm
    ExplainAlgorithm {
        /// Algorithm name
        algorithm: String,
    },
    /// Generate test cases
    GenerateTest {
        /// Code or description
        code: String,
        /// Programming language
        #[arg(long)]
        language: Option<String>,
    },
    /// List and analyze directory contents
    ListDir {
        /// Directory path
        path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate arguments before initializing client
    match &cli.command {
        Commands::Summarize { file, stdin } => {
            if !*stdin && file.is_none() {
                return Err(GmError::InvalidInput("Either --stdin or a file path must be provided".to_string()));
            }
        }
        _ => {}
    }

    // Initialize Gemini client
    let gemini_client = gemini::RealGeminiClient::new()?;

    match cli.command {
        Commands::Explain { command } => {
            let explain_command = commands::explain::ExplainCommand::new(&gemini_client);
            let explanation = explain_command.generate_content(&command).await?;
            println!("{}", explanation);
            Ok(())
        },
        Commands::Summarize { file, stdin } => commands::summarize::execute(&gemini_client, file, stdin).await,
        Commands::Translate { text, to } => commands::translate::execute(gemini_client, text, to).await,
        Commands::Define { term, context } => commands::define::execute(gemini_client, term, context).await,
        Commands::Brainstorm { topic } => commands::brainstorm::execute(gemini_client, topic).await,
        Commands::Rephrase { text, style: _ } => commands::rephrase::execute(gemini_client, text).await,
        Commands::FixTypos { text } => commands::fix_typos::execute(gemini_client, text).await,
        Commands::GenerateAlias { description } => commands::generate_alias::execute(gemini_client, description).await,
        Commands::AnalyzeSentiment { text, stdin } => commands::analyze_sentiment::execute(gemini_client, text, stdin).await,
        Commands::ExplainCode { file, language } => commands::explain_code::execute(gemini_client, file, language).await,
        Commands::ExplainError { error, stdin } => commands::explain_error::execute(gemini_client, error, stdin).await,
        Commands::GenerateRegex { pattern } => commands::generate_regex::execute(gemini_client, pattern).await,
        Commands::ConvertFormat { file, stdin, from, to } => commands::convert_format::execute(gemini_client, file, stdin, from, to).await,
        Commands::ExplainConcept { concept, field } => commands::explain_concept::execute(gemini_client, concept, field).await,
        Commands::FindCommand { task } => commands::find_command::execute(gemini_client, task).await,
        Commands::GenerateBoilerplate { description } => commands::generate_boilerplate::execute(gemini_client, description).await,
        Commands::ReviewText { text, stdin, criteria } => commands::review_text::execute(gemini_client, text, stdin, criteria).await,
        Commands::SuggestName { description } => commands::suggest_name::execute(gemini_client, description).await,
        Commands::ExplainAlgorithm { algorithm } => commands::explain_algorithm::execute(gemini_client, algorithm).await,
        Commands::GenerateTest { code, language } => commands::generate_test::execute(gemini_client, code, language).await,
        Commands::ListDir { path } => commands::list_dir::execute(gemini_client, path).await,
    }
}
