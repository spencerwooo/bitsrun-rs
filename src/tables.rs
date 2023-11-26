use crate::user::enumerate_config_paths;
use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::Style;

pub fn print_config_paths() {
    println!("{} list of possible config paths", "bitsrun:".blue());

    let mut builder = Builder::default();
    builder.set_header(["Priority", "Possible config path"]);

    for (i, path) in enumerate_config_paths().iter().enumerate() {
        builder.push_record([(i + 1).to_string(), path.into()]);
    }

    let mut table = builder.build();
    println!("{}", table.with(Style::sharp()));
}
