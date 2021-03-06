use crate::cmds::Config;
use crate::util::{App, CliArgs, CliError, CliResult};
use clap::Arg;
use pica::Record;

use std::collections::HashMap;
use std::io::BufRead;

pub fn cli() -> App {
    App::new("frequency")
        .about("Compute a frequency table of a subfield.")
        .arg(
            Arg::new("skip-invalid")
                .short('s')
                .long("skip-invalid")
                .about("skip invalid records"),
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("--limit")
                .value_name("n")
                .about("Limit the result to the <n> most common items.")
                .default_value("0"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("--output")
                .value_name("file")
                .about("Write output to <file> instead of stdout."),
        )
        .arg(Arg::new("path").required(true))
        .arg(Arg::new("filename"))
}

pub fn run(args: &CliArgs) -> CliResult<()> {
    let ctx = Config::new();
    let skip_invalid = args.is_present("skip-invalid");
    let limit: u64 = args.value_of("limit").unwrap().parse().unwrap();
    let path_str = args.value_of("path").unwrap();
    let reader = ctx.reader(args.value_of("filename"))?;
    let writer = ctx.writer(args.value_of("output"))?;
    let mut writer = csv::Writer::from_writer(writer);

    let mut ftable: HashMap<String, u64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(record) = Record::decode(&line) {
            for value in record.path(path_str) {
                *ftable.entry(String::from(value)).or_insert(0) += 1;
            }
        } else if !skip_invalid {
            return Err(CliError::Other(format!(
                "could not read record: {}",
                line
            )));
        }
    }

    let mut ftable_sorted: Vec<(&String, &u64)> = ftable.iter().collect();
    ftable_sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (value, frequency) in ftable_sorted {
        if *frequency < limit {
            break;
        }
        writer.write_record(&[value, &frequency.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}
