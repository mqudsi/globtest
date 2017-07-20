extern crate clap;
extern crate glob;

fn main() {
    let app = clap::App::new("globtest")
        .version("0.1")
        .author("NeoSmart Technologies <https://neosmart.net/>")
        .about("Print the results of rs-glob expressions")
        .arg(clap::Arg::with_name("glob")
             .value_name("GLOB")
             .required(true)
             .multiple(true))
        .arg(clap::Arg::with_name("all")
             .takes_value(false)
             .short("a")
             .long("--")
             .help("Include hidden directories and files"));

    let matches = app.get_matches();
    let globs = matches.values_of_lossy("glob").unwrap();
    let include_hidden = matches.is_present("all");

    let options = glob::MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: !include_hidden,
    };

    for glob in globs {
        for entry in glob::glob_with(&glob, &options).unwrap() {
            match entry {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
