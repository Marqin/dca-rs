use std::env;
use clap::{Arg, App};

use ::VERSION;

pub fn parse_args<'a>() -> ::clap::ArgMatches<'a> {

    fn is_within_bitrate_range(v: String) -> Result<(), String> {
        let value = v.parse::<u32>().unwrap();
        if (value < 1) || (value > 512) {
            Err(String::from("Bitrate provided is not valid, it must be between 8 and 128 kb/s"))
        } else {
            Ok(())
        }
    }

    let mut args: Vec<String> = env::args().collect();

    let app = App::new("dca-rs")
                      .version(VERSION)
                      .arg(Arg::with_name("application")
                        .short("a").long("aa")
                        .help("audio application can be voip, audio, or lowdelay")
                        .takes_value(true)
                        .value_name("application")
                        .possible_values(&["voip", "audio", "lowdelay"])
                        .default_value("audio"))
                      .arg(Arg::with_name("bitrate")
                        .short("b").long("ab")
                        .help("audio encoding bitrate in kb/s can be 8 - 128")
                        .takes_value(true)
                        .value_name("bitrate")
                        .default_value("64")
                        .validator(is_within_bitrate_range))
                      .arg(Arg::with_name("channels")
                        .short("c").long("ac")
                        .help("audio channels, 1 for mono 2 for stereo")
                        .takes_value(true)
                        .value_name("channels")
                        .default_value("2")
                        .possible_values(&["1", "2"]))
                      .arg(Arg::with_name("sample rate")
                        .short("r").long("ar")
                        .help("audio sample rate")
                        .takes_value(true)
                        .value_name("sampling rate")
                        .default_value("48000"))
                      .arg(Arg::with_name("frame size")
                        .short("s").long("as")
                        .help("audio frame size can be 960 (20ms), 1920 (40ms), or 2880 (60ms)")
                        .takes_value(true)
                        .value_name("frame size")
                        .default_value("960")
                        .possible_values(&["960", "1920", "2880"]))
                      /*.arg(Arg::with_name("cover format")
                        .short("f").long("cf")
                        .help("format the cover art will be encoded with")
                        .takes_value(true)
                        .value_name("format")
                        .default_value("jpeg"))*/
                      .arg(Arg::with_name("input")
                        .short("i").long("i")
                        .help("input file")
                        .takes_value(true)
                        .value_name("file")
                        .required(true)
                        .default_value("pipe:0"))
                      .arg(Arg::with_name("volume")
                        .short("v")
                        .long("vol")
                        .help("change audio volume (256=normal)")
                        .takes_value(true)
                        .value_name("level")
                        .default_value("256"))
                      /*.arg(Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("specify an output file (leave blank for stdout output)")
                        .takes_value(true)
                        .value_name("file"))*/
                      .arg(Arg::with_name("raw")
                        .long("raw")
                        .help("raw opus output (no metadata or magic bytes)"));

    if args.len() == 1 {
        app.print_help().unwrap();
        ::std::process::exit(0);
    }

    if args.len() == 2 {
        // assume single argument is a file
        args.insert(1, "--i".to_owned());
    }

    app.get_matches_from(args)
}