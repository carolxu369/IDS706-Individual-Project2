use clap::{Arg, App, SubCommand};
use my_crate::db;
use my_crate::error::CliError;
use rusqlite::Connection;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    db::create_table(&conn)?;

    let matches = App::new("My CLI")
        .version("1.0")
        .author("Your Name. <your_email@example.com>")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new record")
                .arg(
                    Arg::with_name("DATA")
                        .help("The data to be inserted")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("Reads all records"),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates an existing record")
                .arg(
                    Arg::with_name("ID")
                        .help("The ID of the record to update")
                        .required(true),
                )
                .arg(
                    Arg::with_name("DATA")
                        .help("The new data for the record")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Deletes a record")
                .arg(
                    Arg::with_name("ID")
                        .help("The ID of the record to delete")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("create", Some(sub_m)) => {
            let data = sub_m
                .value_of("DATA")
                .ok_or(CliError::ArgumentError("Missing data for create".into()))?;
            db::create_entry(&conn, data)?;
        }
        ("read", Some(_)) => {
            db::read_entries(&conn)?;
        }
        ("update", Some(sub_m)) => {
            let id_str = sub_m
                .value_of("ID")
                .ok_or(CliError::ArgumentError("Missing ID for update".into()))?;
            let id: i32 = id_str.parse()?;
            let data = sub_m
                .value_of("DATA")
                .ok_or(CliError::ArgumentError("Missing data for update".into()))?;
            db::update_entry(&conn, id, data)?;
        }
        ("delete", Some(sub_m)) => {
            let id_str = sub_m
                .value_of("ID")
                .ok_or(CliError::ArgumentError("Missing ID for delete".into()))?;
            let id: i32 = id_str.parse()?;
            db::delete_entry(&conn, id)?;
        }
        _ => return Err(Box::new(CliError::ArgumentError("Invalid command".into()))),
    }

    Ok(())
}
