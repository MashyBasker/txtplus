/*
Implementation of a textbox ASCII representation based on the directive => [string]
Any string not enclosed in "[]" will be ignored
Example:
====================
@@start::box
[Hello]
@@end::box

will render to
+-------+
| Hello |
+-------+
===================

@@start::box
[Hello] [World]
@@start::end

+-------+  +-------+
| Hello |  | World |
+-------+  +-------+
====================

@@start::box
[Good]
[Morning]
@@end::box

+------+
| Good |
+------+
+---------+
| Morning |
+---------+
=================
Long sentences will be broken down into multiple line,

@@start::box
[An idiot admires complexity, a genius admires simplicity]
@@end::box

+----------------------+
| An idiot admires     |
| complecity, a genius |
| admires simplicity   |
+----------------------+

*/

use std::io::Write;
use std::{fs::OpenOptions, iter::repeat};

use regex::Regex;

fn get_box_string(directive_line: &String) -> Vec<&str> {
    let re = Regex::new(r"\[([^\]]*)\]").unwrap();
    re.captures_iter(directive_line)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect()
}

fn create_textbox_string(strings: Vec<String>) -> Vec<String> {
    strings
        .into_iter()
        .map(|string| {
            let mut s_box = String::new();
            let n = string.split("\n").map(|x| x.len()).max().unwrap_or(0);
            let border = format!("+{:-<width$}+\n", "", width = n + 2);
            for s in string.split("\n") {
                s_box.push_str(
                    format!(
                        "| {}{} |\n",
                        s,
                        repeat(' ').take(n - s.len()).collect::<String>()
                    )
                    .as_str(),
                );
            }
            format!("{}{}{}", border, s_box, border.trim_end())
        })
        .collect()
}

fn box_to_file(fpath: &String, textboxes: Vec<String>) -> std::io::Result<()> {
    let split_boxes: Vec<Vec<String>> = textboxes
        .iter()
        .map(|s| s.split('\n').map(|l| l.to_string()).collect())
        .collect();

    let max_len = split_boxes
        .iter()
        .map(|bxline| bxline.len())
        .max()
        .unwrap_or(0);

    let mut f = OpenOptions::new().append(true).open(fpath)?;
    for i in 0..max_len {
        let line = split_boxes
            .iter()
            .map(|bxline| {
                if i < bxline.len() {
                    bxline[i].clone()
                } else {
                    " ".repeat(bxline[0].len())
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(f, "{}", line)?;
    }
    Ok(())
}

fn newline_string(strings: Vec<&str>) -> Vec<String> {
    strings
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .collect::<Vec<&str>>()
                .chunks(3)
                .map(|chunk| chunk.join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
}

fn draw_texbox(file: &String, cmd_line: &String) -> std::io::Result<()> {
    let texts = get_box_string(cmd_line);
    let newline_splitted = newline_string(texts);
    let textboxes = create_textbox_string(newline_splitted);
    box_to_file(file, textboxes)?;
    Ok(())
}

#[allow(unused)]
pub fn render_box(file: &String, directive_cmd: &mut Vec<String>) {
    // println!("{:?}", directive_cmd);
    let n = directive_cmd.len();
    for i in 0..n - 1 {
        draw_texbox(file, &directive_cmd[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_box_string() {
        assert_eq!(
            get_box_string(&"[hello] [world]".to_string()),
            vec!["hello", "world"]
        );
        assert_eq!(
            get_box_string(&"[boxy] [boxy] random".to_string()),
            vec!["boxy", "boxy"]
        );
    }

    #[test]
    fn test_create_textbox_string() {
        assert_eq!(
            create_textbox_string(vec!["hello".to_string(), "world".to_string()]),
            vec![
                "+-------+\n| hello |\n+-------+",
                "+-------+\n| world |\n+-------+"
            ]
        );
        assert_eq!(
            create_textbox_string(vec!["multiline\nstring\nhere".to_string()]),
            vec!["+-----------+\n| multiline |\n| string    |\n| here      |\n+-----------+"]
        );
    }
}
