use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Dewa Sahu",
    about = "A Command-line tool to analyze lyrics to songs and put them into a sqlite database."
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Dewa sahu")]
    Classify {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
    Candidates {},
    Lyrics {
        #[clap(short, long, default_value = "lyrics.txt")]
        file: String,
    },
}

fn main(){
    let args=Cli::parse()
    
}