use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("zz")
        .version("alpha")
        .author("github.com")
        .about("")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            // argument style
            // x in
            // x in out
            // x -- in out (for hyphened paths)
            SubCommand::with_name("x")
                .visible_aliases(&["e", "decompress", "extract"])
                .arg(
                    Arg::with_name("in")
                        .help("input file/dir path")
                        .required(true)
                        .index(1),
                )
                .arg(Arg::with_name("out").help("out file/dir path").index(2)), // .arg(Arg::with_name("no-extra-dir")),
        )
        .subcommand(
            // argument style
            // x 1 2 3/ (defaults to zip)
            // x 1 2 out.gz
            // x out.gz 1 2
            SubCommand::with_name("c")
                .visible_aliases(&["a", "compress", "archive"])
                .arg(
                    Arg::with_name("path")
                        .multiple(true)
                        .help("A list of file/directories to compress"),
                )
                .arg(
                    Arg::with_name("out")
                        .long("out")
                        .short("out")
                        .help("A output filename"),
                ),
        )
}

fn main() {
    let args = build_cli().get_matches();
    let (sub, matches) = args.subcommand();
    let matches = matches.unwrap();

    match sub {
        "x" => {
            let in_ = matches.value_of("in").unwrap();
            let out = matches.value_of("out");
            zz::decompress_once(in_, out).unwrap();
        }
        "c" => {
            // argument style
            // 1. single directory => dir_name as archive name + strip dir
            // 2. multiple arguments => random name as archive_name + all under archive root
            todo!();
            // if let Some() = matches.value_of("a") {}
        }
        _ => todo!(),
    }
}
