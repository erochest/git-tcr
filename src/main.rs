use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg,
    ArgMatches,
};

fn main() {
    let matches = build_app().get_matches();
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!().arg(
        Arg::with_name("command")
            .short("c")
            .long("command")
            .value_name("COMMAND")
            .help("The command to run."),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_app_takes_command() {
        let app = build_app();
        let args = app.get_matches_from(&["git-tcr", "--command=echo hello world"]);
        assert_that(&args.value_of("command"))
            .is_some()
            .is_equal_to(&"echo hello world");
    }
}
