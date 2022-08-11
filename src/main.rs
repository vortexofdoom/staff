use clap::{ArgEnum, Parser, Subcommand};
use staff::{
    note::{Accidental, Flat},
    Chord, Natural, Note, Scale,
};

#[derive(Parser)]
#[clap(author, version, about = "Music theory command-line interface", long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Display a chord's notes
    Chord {
        /// Name (symbol) of the chord
        name: String,
    },

    /// Display a scale's notes
    Scale {
        /// Root note of the scale
        root: String,

        /// Mode of the scale
        #[clap(arg_enum, value_parser)]
        mode: Mode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Major,
    Minor,
    Ionian,
    Aeolian,
    Dorian,
}

fn print_scale<A: Accidental>(root_note: Note<A>, mode: Mode) {
    let scale = match mode {
        Mode::Major | Mode::Ionian => Scale::major(root_note),
        Mode::Minor | Mode::Aeolian => Scale::natural_minor(root_note),
        Mode::Dorian => Scale::dorian(root_note),
    };
    let mut iter = scale.peekable();
    while let Some(note) = iter.next() {
        print!("{}", note);
        if iter.peek().is_some() {
            print!(" ");
        } else {
            println!()
        }
    }
}

fn main() {
    let cli = App::parse();
    match &cli.command {
        Command::Chord { name } => {
            let chord: Chord = name.parse().unwrap();
            let mut iter = chord.into_iter().peekable();
            while let Some(note) = iter.next() {
                print!("{}", note);
                if iter.peek().is_some() {
                    print!(" ");
                } else {
                    println!()
                }
            }
        }
        Command::Scale { root, mode } => {
            let mut chars = root.chars();
            let natural: Natural = chars.next().unwrap().try_into().unwrap();

            match chars.next() {
                Some('b') => {
                    let root_note = match chars.next() {
                        Some('b') => Note::double_flat(natural),
                        None => Note::flat(natural),
                        _ => todo!(),
                    };
                    print_scale(root_note, *mode);
                }
                Some('#') => {
                    let root_note = match chars.next() {
                        Some('#') => Note::double_sharp(natural),
                        None => Note::sharp(natural),
                        _ => todo!(),
                    };
                    print_scale(root_note, *mode);
                }
                None => {
                    let root_note: Note<Flat> = natural.into();
                    print_scale(root_note, *mode);
                }
                _ => todo!(),
            };
        }
    }
}
