extern crate clap;
use self::clap::{Arg, ArgMatches, App};
pub fn utargs() -> ArgMatches<'static> {
    App::new("untod")
        .version("0.0.1")
        .author("Brent")
        .about("Converts between extended TOD, Date/Time, and Perpetual Minute Clock values")
        .arg(Arg::with_name("leapsec")
             .help("Adjust for TAI leap-seconds")
             .long("leapseconds")
             .short("l")
             .takes_value(false))
        .arg(Arg::with_name("ng")
             .help("No GMT: suppress GMT result if others given")
             .long("ng")
             .takes_value(false))
        .arg(Arg::with_name("clipboard")
             .help("Get values for conversion from clipboard")
             .short("c")
             .long("clipboard")
             .conflicts_with("values")
             .takes_value(false))
        .arg(Arg::with_name("pl")
             .help("Pad Left: pad TOD with 0 on left")
             .long("pl")
             .takes_value(false))
        .arg(Arg::with_name("pr")
             .conflicts_with("pl")
             .help("Pad Right: pad TOD with 0 on right")
             .long("pr")
             .takes_value(false))
        .arg(Arg::with_name("zl")
             .help("Local timezone: override local time offset ([-+]n.n)")
             .long("zl")
             .env("TODL")
             .takes_value(true)
             .value_name("offset"))
        .arg(Arg::with_name("za")
             .help("Alternate timezone: specify additional timezone offset ([-+]n.n)")
             .long("za")
             .env("TODA")
             .takes_value(true)
             .value_name("offset"))
        .arg(Arg::with_name("pmc")
             .help("Convert from Perpetual Minute Clock values")
             .short("m")
             .long("pmc")
             .takes_value(false))
        .arg(Arg::with_name("reverse")
             .conflicts_with("pmc")
             .help("Convert from Date/Time values")
             .short("r")
             .long("rev")
             .takes_value(false))
        .arg(Arg::with_name("values")
             .help("Values for conversion")
             .takes_value(true)
             .required_unless("clipboard")
             .default_value("NOW")
             .multiple(true))
        .get_matches()
}
