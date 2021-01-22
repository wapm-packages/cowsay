#[macro_use]
extern crate include_dir;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use include_dir::Dir;

pub const PROJECT_DIR: Dir = include_dir!("src/cows/");

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

pub fn list_cows() -> Vec<String> {
    PROJECT_DIR
        .files()
        .iter()
        .map(|file| file.path.replace(".cow", ""))
        .collect::<Vec<String>>()
}

pub fn format_animal(s: String, thoughts: &str, eyes: &str, tongue: &str) -> String {
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

pub fn make_bubble(s: String, width: usize, think: bool, wrap: bool) -> String {
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
        botleft: "\\",
        topright: "\\",
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

            let mut subindex = index + width;
            let localwidth = loop {
                match (&s[index..subindex]).ends_with(" ") {
                    true => break subindex - index,
                    false => subindex -= 1,
                }
                if index == subindex {
                    break width;
                }
            };
            let slice = &s[index..index + localwidth];
            result.push(slice.to_string());
            index += localwidth;
        }
    }
    let slice = &s[index..];
    result.push(slice.to_string());

    // Bookend lines with bubble chars
    let mut longest = 0;
    let reslen = result.len() - 1;
    for (index, line) in result.iter_mut().enumerate() {
        match index {
            0 => match reslen {
                0 | 1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = vec![cowb.topleft, line, cowb.topright].join(" "),
            },
            x if x < reslen => *line = vec![cowb.midleft, line, cowb.midright].join(" "),
            y if y == reslen => match reslen {
                1 => *line = vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => *line = vec![cowb.botleft, line, cowb.botright].join(" "),
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
