use clap::{App, Arg, SubCommand};
use git2::{Error, Repository};
use prettytable::{Cell, Row, Table};

fn main() -> Result<(), Error> {
    let app_m = App::new("Jim")
        .version("0.0.1")
        .author("Cameron Dart <cdart2@illinois.edu>")
        .about("Your buddy Jim is always there for you.")
        .subcommand(
            SubCommand::with_name("history").arg(
                Arg::with_name("repo")
                    .required(true)
            ).help("List commit history of repository found at provided path")
        )
        .get_matches();

    match app_m.subcommand() {
        ("history", Some(arg_m)) => display_history(arg_m.value_of("repo").unwrap()).unwrap(),
        _ => println!("{}", app_m.usage()),
    }
    Ok(())
}

fn display_history(repo: &str) -> Result<(), Error> {
    let repo = Repository::open(repo)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL)?;
    revwalk.push(repo.head().unwrap().target().unwrap())?;

    let mut table = Table::new();
    table.add_row(
        vec!["author", "summary", "hash"]
            .iter()
            .map(|val| Cell::new(val))
            .collect(),
    );

    // hardcoded limit for now...
    let mut limit = 30;
    for moid in revwalk {
        limit = limit - 1;
        let oid = moid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();
        let email = author.email().unwrap_or("Unknown Author");
        let summary = commit.summary().unwrap_or("No Commit Message");
        table.add_row(Row::new(
            vec![email, summary, &format!("{}", oid)]
                .iter()
                .map(|val| Cell::new(val))
                .collect(),
        ));
        if limit < 0 {
            break;
        }
    }
    table.printstd();
    Ok(())
}
