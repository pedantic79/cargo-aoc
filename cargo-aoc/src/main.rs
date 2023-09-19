/// TODO: refactor this. As of Rust 2018 Edition, extern crate is no longer required.
extern crate aoc_runner_internal;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
extern crate directories;
/// TODO: Do we actually need to rely on reqwest ?
/// ... Tokio is overkill for the scope of this project.
extern crate reqwest;
extern crate toml;
extern crate webbrowser;

mod app;
mod credentials;
mod date;
mod project;

use clap::{Arg, ArgAction, Command};

use app::AOCApp;

fn main() {
    // Parses the attributes (CLAP)
    let matches = Command::new("cargo-aoc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Cargo helper for Advent of Code")
        .author("gobanos <gregory.obanos@gmail.com>")
        .arg(Arg::new("dummy").hide(true).value_parser(["aoc"]))
        .arg(
            Arg::new("day")
                .short('d')
                .help("Specifies the day. Defaults to last implemented.")
                .num_args(1),
        )
        .arg(
            Arg::new("part")
                .short('p')
                .help("Specifies the part. Defaults to both parts.")
                .num_args(1),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .help("Use an alternate input file.")
                .num_args(1),
        )
        .arg(
            Arg::new("profile")
                .short('x')
                .help("Add debug info for profiling tools.")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("bench")
                .about("Benchmark your solutions")
                .arg(
                    Arg::new("day")
                        .short('d')
                        .help("Specifies the day. Defaults to last implemented.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("part")
                        .short('p')
                        .help("Specifies the part. Defaults to both parts.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .help("Use an alternate input file.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("open")
                        .short('o')
                        .help("Opens the benchmark information in the browser")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("generator")
                        .short('g')
                        .help("Also benchmark generator functions.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("profile")
                        .short('x')
                        .help("Add debug info for profiling tools.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("credentials")
                .about("Manage your AOC credentials information")
                .arg(
                    Arg::new("set")
                        .short('s')
                        .help("Sets the session cookie")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("input")
                .about("Get the input for a specified date")
                .arg(
                    Arg::new("day")
                        .short('d')
                        .help("Specifies the day. Defaults to today's date.")
                        .num_args(1),
                )
                .arg(
                    Arg::new("year")
                        .short('y')
                        .help("Specifies the year. Defaults to the current year.")
                        .num_args(1),
                ),
        )
        .get_matches();

    // Creates an AOCApp that we'll use to launch actions (commands)
    let app = AOCApp::new();

    match matches.subcommand() {
        Some(("credentials", m)) => app.execute_credentials(m),
        Some(("input", m)) => app.execute_input(m),
        Some(("bench", m)) => {
            if let Err(e) = app.execute_bench(m) {
                eprintln!("An error occurs : {}", e);
                std::process::exit(-1);
            }
        }
        Some((c, _)) => panic!("Unknown command `{}`", c),
        _ => {
            if let Err(e) = app.execute_default(&matches) {
                eprintln!("An error occurs : {}", e);
                std::process::exit(-1);
            }
        }
    }
}
