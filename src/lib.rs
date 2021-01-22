#[macro_use]
extern crate include_dir;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

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
        .trim_end()
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
    let pad = " ";
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
    let mut longest = 0;
    for input_line in s.lines() {
        if wrap {
            let mut line = String::with_capacity(width);
            let mut line_width = 0;
            for word in input_line.split_word_bounds() {
                let word = word.replace("\t", "    ");
                let word_width = word.width();
                if line_width + word_width <= width {
                    line += &word;
                    line_width += word_width;
                } else if word_width < width {
                    result.push((line, line_width));
                    longest = std::cmp::max(line_width, longest);
                    line = String::with_capacity(width);
                    line_width = 0;
                    if ! word.trim_end().is_empty() {
                        line += &word;
                        line_width = word_width;
                    }
                } else {
                    for gc in word.graphemes(true) {
                        let gc_width = UnicodeWidthStr::width(gc);
                        if line_width == 0 || gc_width + line_width <= width {
                            line += gc;
                            line_width += gc_width;
                        } else {
                            result.push((line, line_width));
                            longest = std::cmp::max(line_width, longest);
                            line = String::with_capacity(width);
                            line += gc;
                            line_width = gc_width;
                        }
                    }
                }
            }
            result.push((line, line_width));
            longest = std::cmp::max(line_width, longest);
        } else {
            let width = input_line.width();
            longest = width;
            result.push((input_line.to_string(), width));
        }
    }
    if result.len() == 0 {
        result.push(("".to_string(), 0));
    }

    // Pad to longest line
    for (line, line_width) in &mut result {
        *line += &pad.repeat(longest - *line_width)
    }

    // Bookend lines with bubble chars
    let reslen = result.len() - 1;
    let result = result.iter_mut().enumerate().map(|(index, (line, _))| {
        let line = match index {
            0 => match reslen {
                0 | 1 => vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => vec![cowb.topleft, line, cowb.topright].join(" "),
            },
            x if x < reslen => vec![cowb.midleft, line, cowb.midright].join(" "),
            y if y == reslen => match reslen {
                1 => vec![cowb.sleft, line, cowb.sright].join(" "),
                _ => vec![cowb.botleft, line, cowb.botright].join(" "),
            },
            _ => panic!("Whoops!"),
        };
        line
    }).collect::<Vec<_>>();

    let mut top_bottom = longest + 2;
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
    use std::iter::once;
    once(top.join(""))
        .chain(result.into_iter())
        .chain(once(bottom.join("")))
        .collect::<Vec<_>>()
        .join("\n")
}
