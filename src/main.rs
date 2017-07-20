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
             .long("all")
             .help("Include hidden directories and files"))
        .arg(clap::Arg::with_name("nocase")
             .takes_value(false)
             .short("i")
             .long("nocase")
             .help("Disable case sensitivity"))
        .arg(clap::Arg::with_name("debug")
             .takes_value(false)
             .short("d")
             .long("debug")
             .help("Enable debug output"));

    let matches = app.get_matches();
    let globs = matches.values_of_lossy("glob").unwrap();
    let include_hidden = matches.is_present("all");
    let case_sensitive = !matches.is_present("nocase");
    let debug = matches.is_present("debug");

    let options = glob::MatchOptions {
        case_sensitive: case_sensitive,
        require_literal_leading_dot: !include_hidden,
        require_literal_separator: true,
    };

    if debug {
        println!("");
        println!("\toptions {{");
        println!("\t\tcase_sensitive: {},", options.case_sensitive);
        println!("\t\trequire_literal_leading_dot: {},", options.require_literal_leading_dot);
        println!("\t\trequire_literal_separator: {}", options.require_literal_separator);
        println!("\t}};");
        println!("");
    }

    for glob in globs {
        for entry in glob::glob_with(&glob, &options).unwrap() {
            match entry {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
