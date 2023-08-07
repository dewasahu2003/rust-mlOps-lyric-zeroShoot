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

fn main() {
    
    let args = Cli::parse();
    match args.command {
        Some(Commands::Classify { file }) => {
            println!("Classify {}", file);
            let lyrics = lyric::read_lyrics(&file);
            let result = lyric::classify_lyrics(lyrics);
            for label in result {
                for l in label {
                    println!("{}: {}", l.text, l.score)
                }
            }
        }
        Some(Commands::Candidates {}) => {
            for candidate in lyric::get_all_zeroshotCandidate() {
                println!("{}", candidate);
            }
        }
        Some(Commands::Lyrics { file }) => {
            println!("Lyrics: {}", file);
            for line in lyric::read_lyrics(&file) {
                println!("{}", line);
            }
        }
        None => {
            println!("No command found")
        }
    }
}
