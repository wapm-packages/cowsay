extern crate clap;
extern crate rand;

use std::str;
use std::env;
use std::io::{self, Read};
use std::fs::File;
use clap::{App, Arg};
use rand::{thread_rng, sample};

mod assets;


struct CowBubble {
    sleft: &'static str,
    sright: &'static str,
    topleft: &'static str,
    midleft: &'static str,
    botleft: &'static str,
    topright: &'static str,
    midright: &'static str,
    botright: &'static str,
}


fn list_cows() -> Vec<String> {
    assets::list()
        .iter()
        .map(|x| x.split("/").last().unwrap().replace(".cow", ""))
        .collect::<Vec<String>>()
}

fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
    s.split("\n")
        .filter(|&x| !x.starts_with("##") && !x.contains("EOC"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_right()
        .replace("$eyes", eyes)
        .replace("$thoughts", thoughts)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\")
        .replace("\\@", "@")
}

fn make_bubble(s: String, width: usize, think: bool, wrap: bool) -> String {
    let mut result = Vec::new();
    let mut top = vec![" "];
    let mut bottom = vec![" "];
    let topc = "_";
    let bottomc = "-";
    let pad = ' ';
    let mut cowb = CowBubble {
        sleft: "<",
        sright: ">",
        topleft: "/",
        midleft: "|",
        botleft : "\\",
        topright : "\\",
        midright: "|",
        botright: "/",
    };


    if think {
        cowb = CowBubble {
            sleft: "(",
            sright: ")",
            topleft: "(",
            midleft: "(",
            botleft: "(",
            topright: ")",
            midright: ")",
            botright: ")",
        };
    }


    // Linewrap
    let mut index = 0;
    if wrap {
        loop {
            if index + width >= s.len() {
                break;
            }

            let localwidth;
            let mut subindex = index + width;
            'b: loop {
                match (&s[index .. subindex]).ends_with(" ") {
                    true => {
                        localwidth = subindex - index;
                        break 'b;
                    }
                    false => {
                        subindex -= 1;
                    }
                }
            }
            let slice = &s[index .. index + localwidth];
            result.push(slice.to_string());
            index += localwidth;
        }
    }
    let slice = &s[index .. ];
    result.push(slice.to_string());


    // Bookend lines with bubble chars
    let mut longest = 0;
    let reslen = result.len() - 1;
    for (index, line) in result.iter_mut().enumerate() {
        match index {
            0 => match reslen {
                0 | 1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => {
                    *line = vec![cowb.topleft, line, cowb.topright].join(" ")
                }
            },
            x if x < reslen => *line = vec![cowb.midleft, line, cowb.midright].join(" "),
            y if y == reslen => match reslen {
                1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = vec![cowb.botleft, line, cowb.botright].join(" ")
            },
            _ => panic!("Whoops!"),
        }
        if line.len() > longest {
            longest = line.len();
        }
    }

    // Pad to longest line
    for line in &mut result {
        let mut padding = longest - line.len();
        let linelen = line.len();
        loop {
            match padding > 0 {
                false => break,
                true => {
                    line.insert(linelen - 1, pad);
                    padding -= 1;
                }
            };
        }
    }

    let mut top_bottom = longest - 2;
    loop {
        match top_bottom > 0 {
            false => break,
            true => {
                top.push(topc);
                bottom.push(bottomc);
                top_bottom -= 1;
            }
        }
    }
    result.insert(0, top.join(""));
    result.push(bottom.join(""));
    result.join("\n")
}

fn main() {

    let matches = App::new("rust-cowsay")
        .version("v0.1.0-pre-alpha")
        .author("Matt Smith. <matthew.smith491@gmail.com>")
        .arg(Arg::with_name("MESSAGE")
            .help("Message for cow to say")
            .multiple(true))
        .arg(Arg::with_name("cow")
            .short("f")
            .value_name("COW")
            .help("Which cow should say"))
        .arg(Arg::with_name("width")
            .short("W")
            .value_name("WIDTH")
            .help("Max width of cow text bubble"))
        .arg(Arg::with_name("nowrap")
            .short("n")
            .help("Disable word wrap"))
        .arg(Arg::with_name("borg")
            .short("b")
            .help("Borg Cow"))
        .arg(Arg::with_name("dead")
            .short("d")
            .help("Dead Cow"))
        .arg(Arg::with_name("greedy")
            .short("g")
            .help("Greedy Cow"))
        .arg(Arg::with_name("paranoid")
            .short("p")
            .help("Paranoid Cow"))
        .arg(Arg::with_name("stoned")
            .short("s")
            .help("Stoned Cow"))
        .arg(Arg::with_name("tired")
            .short("t")
            .help("Tired Cow"))
        .arg(Arg::with_name("wired")
            .short("w")
            .help("Wired Cow"))
        .arg(Arg::with_name("youthful")
            .short("y")
            .help("Youthful Cow"))
        .arg(Arg::with_name("custom")
            .short("e")
            .value_name("EYE_STRING")
            .help("Custom Eyes"))
        .arg(Arg::with_name("tongue")
            .short("T")
            .value_name("TONGUE_STRING")
            .help("Custom Tongue"))
        .arg(Arg::with_name("list")
            .short("l")
            .help("List Cows"))
        .arg(Arg::with_name("random")
            .long("random")
            .help("Choose random cow"))
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
        false => cow
    };

    let width = matches.value_of("width").unwrap_or("40").parse::<usize>().unwrap();
    let wrap = !matches.is_present("nowrap");
    let message_vals = match matches.values_of("MESSAGE") {
        Some(x) => x.collect::<Vec<_>>(),
        None => vec![""]
    };
    let mut message = message_vals.join(" ");

    message = match &message[..] {
        "" => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim_right().to_string()
        },
        _ => message
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


    let eyes = [(borg, "=="),
                (dead, "xx"),
                (greedy, "$$"),
                (paranoid, "@@"),
                (stoned, "**"),
                (tired, "--"),
                (wired, "OO"),
                (youthful, ".."),
                (custombool, custom),
                (true, "oo")]
                    .iter()
                    .filter(|&x| x.0)
                    .collect::<Vec<_>>()[0].1;

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
            let mut f = File::open(&cow).unwrap();
            f.read_to_string(&mut cowbody).expect(
                &format!("Couldn't read cowfile {}", cow));
        },
        false => {
            let fmt = &format!("src/cows/{}.cow", &cow);
            cowbody = str::from_utf8(assets::get(&fmt).unwrap()).unwrap().to_string();
        }
    }


    println!("{}", make_bubble(message.to_string(), width, think, wrap));
    println!("{}", format_animal(cowbody, voice, eyes, tongue));

}
