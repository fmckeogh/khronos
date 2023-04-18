use {
    clap::{Parser, Subcommand},
    color_eyre::eyre::Result,
    jsonwebtoken::EncodingKey,
    khronos::auth::{create_token, AuthLevel},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Issues a new JWT token
    Issue {
        /// Base64 secret
        #[arg(long)]
        secret: String,
        /// Privilege level
        #[arg(long, value_enum)]
        level: AuthLevel,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Command::Issue { secret, level } => {
            let key = EncodingKey::from_base64_secret(&secret)?;
            let token = create_token(&key, level)?;
            println!(r#"-H "Authorization: Bearer {token}""#)
        }
    }

    Ok(())
}
