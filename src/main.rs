extern crate cowsay;
extern crate clap;
extern crate rand;
use clap::{App, Arg};
use rand::{sample, thread_rng};
use cowsay::*;
use std::env;
use std::io::{self, Read};
use std::str;

fn main() {
    let matches = App::new("cowsay")
        .version("v0.1.0")
        .author("Syrus Akbary <syrus@wasmer.io>")
        .arg(
            Arg::with_name("MESSAGE")
                .help("Message for cow to say")
                .multiple(true),
        )
        .arg(
            Arg::with_name("cow")
                .short("f")
                .value_name("COW")
                .help("Which cow should say"),
        )
        .arg(
            Arg::with_name("width")
                .short("W")
                .value_name("WIDTH")
                .help("Max width of cow text bubble"),
        )
        .arg(
            Arg::with_name("nowrap")
                .short("n")
                .help("Disable word wrap"),
        )
        .arg(Arg::with_name("borg").short("b").help("Borg Cow"))
        .arg(Arg::with_name("dead").short("d").help("Dead Cow"))
        .arg(Arg::with_name("greedy").short("g").help("Greedy Cow"))
        .arg(Arg::with_name("paranoid").short("p").help("Paranoid Cow"))
        .arg(Arg::with_name("stoned").short("s").help("Stoned Cow"))
        .arg(Arg::with_name("tired").short("t").help("Tired Cow"))
        .arg(Arg::with_name("wired").short("w").help("Wired Cow"))
        .arg(Arg::with_name("youthful").short("y").help("Youthful Cow"))
        .arg(
            Arg::with_name("custom")
                .short("e")
                .value_name("EYE_STRING")
                .help("Custom Eyes"),
        )
        .arg(
            Arg::with_name("tongue")
                .short("T")
                .value_name("TONGUE_STRING")
                .help("Custom Tongue"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List Cows"),
        )
        .arg(
            Arg::with_name("random")
                .long("random")
                .help("Choose random cow"),
        )
        .get_matches();

    match matches.is_present("list") {
        true => {
            let list = list_cows();
            println!("{:?}", list);
            std::process::exit(0);
        }
        false => (),
    };

    let mut cow = matches.value_of("cow").unwrap_or("default").to_owned();

    cow = match matches.is_present("random") {
        true => {
            let mut rng = thread_rng();
            let cows = list_cows();
            sample(&mut rng, cows, 1).first().unwrap().to_owned()
        }
        false => cow,
    };

    let width = matches
        .value_of("width")
        .unwrap_or("40")
        .parse::<usize>()
        .unwrap();
    let wrap = !matches.is_present("nowrap");
    let message_vals = match matches.values_of("MESSAGE") {
        Some(x) => x.collect::<Vec<_>>(),
        None => vec![""],
    };
    let mut message = message_vals.join(" ");

    message = match &message[..] {
        "" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim_right().to_string()
        }
        _ => message,
    };

    let tongue = matches.value_of("tongue").unwrap_or(" ");

    // Cow Eyes
    let borg = matches.is_present("borg");
    let dead = matches.is_present("dead");
    let greedy = matches.is_present("greedy");
    let paranoid = matches.is_present("paranoid");
    let stoned = matches.is_present("stoned");
    let tired = matches.is_present("tired");
    let wired = matches.is_present("wired");
    let youthful = matches.is_present("youthful");
    let custom = matches.value_of("custom").unwrap_or("");
    let mut custombool = false;

    if custom != "" {
        custombool = true;
    }

    let eyes = [
        (borg, "=="),
        (dead, "xx"),
        (greedy, "$$"),
        (paranoid, "@@"),
        (stoned, "**"),
        (tired, "--"),
        (wired, "OO"),
        (youthful, ".."),
        (custombool, custom),
        (true, "oo"),
    ]
    .iter()
    .filter(|&x| x.0)
    .collect::<Vec<_>>()[0]
        .1;

    let think;
    let voice;
    match env::args().collect::<Vec<_>>()[0].ends_with("cowthink") {
        true => {
            think = true;
            voice = "o"
        }
        false => {
            think = false;
            voice = "\\";
        }
    }

    let mut cowbody = String::new();

    match cow.contains(".cow") {
        true => {
            unimplemented!("Can't provide external cowfiles for now")
            // let mut f = File::open(&cow).unwrap();
            // f.read_to_string(&mut cowbody)
            //     .expect(&format!("Couldn't read cowfile {}", cow));
        }
        false => {
            let fmt = &format!("{}.cow", &cow);
            let file = PROJECT_DIR
                .get_file(&fmt)
                .expect(&format!("Can't find the cow file {}", cow));
            cowbody = str::from_utf8(file.contents).unwrap().to_string();
        }
    }

    println!("{}", make_bubble(message.to_string(), width, think, wrap));
    println!("{}", format_animal(cowbody, voice, eyes, tongue));
}
