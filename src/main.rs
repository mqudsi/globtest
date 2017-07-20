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
             .multiple(true));

    let matches = app.get_matches();
    let globs = matches.values_of_lossy("glob").unwrap();

    for glob in globs {
        for entry in glob::glob(&glob).unwrap() {
            match entry {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
