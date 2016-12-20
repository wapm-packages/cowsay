extern crate clap;

use std::str;
use std::env;
use clap::{App, Arg};

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

fn format_animal(s: String, thoughts: &str, tongue: &str) -> String {
    s.split("\n")
        .filter(|&x| !x.starts_with("##") && !x.contains("EOC"))
        .collect::<Vec<_>>()
        .join("\n")
        .replace("$eyes", "oo")
        .replace("$thoughts", thoughts)
        .replace("$tongue", tongue)
        .replace("\\\\", "\\")
        .replace("\\@", "@")
}

fn make_bubble(s: String, width: usize, think: bool) -> String {
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
    let slice = &s[index .. ];
    result.push(slice.to_string());


    // Bookend lines with bubble chars
    let mut longest = 0;
    let reslen = result.len() - 1;
    for (index, line) in result.iter_mut().enumerate() {
        match index {
            0 => match reslen {
                1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
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
            .required(true)
            .multiple(true))
        .arg(Arg::with_name("f")
            .short("f")
            .value_name("COW")
            .help("Which cow should say"))
        .arg(Arg::with_name("W")
            .short("W")
            .value_name("WIDTH")
            .help("Max width of cow text bubble"))
        .get_matches();

    let cow = matches.value_of("f").unwrap_or("default");
    let width = matches.value_of("W").unwrap_or("40").parse::<usize>().unwrap();
    let message_vals = matches.values_of("MESSAGE").unwrap().collect::<Vec<_>>();
    let message = message_vals.join(" ");

    let think;

    match env::args().collect::<Vec<_>>()[0].ends_with("cowthink") {
        true => think = true,
        false => think = false,
    }

    let fmt = &format!("src/cows/{}.cow", &cow);
    let mut cowbody = str::from_utf8(assets::get(&fmt).unwrap()).unwrap().to_string();
    cowbody = format_animal(cowbody, "\\", "  ");
    println!("{}", make_bubble(message.to_string(), width, think));
    println!("{}", cowbody);

}
