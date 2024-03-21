/*!
To be used as a git smudge filter,
to replace email addresses in a file with a scrambled version of the email address

```toml
[filter "rot8000"]
    smudge = /home/foo/.cargo/bin/rot8000
    clean = /home/foo/.cargo/bin/rot8000
```

"grfg123@rknzcyr.bet"
"grfg456@rknzcyr.bet"
"grfg456@rknzcyr1.bet"
 !*/

use regex::{Captures, Regex, Replacer};
use std::io::stdin;
struct Rot8000;

impl Replacer for Rot8000 {
    /// Replace the matched text with a replacement.
    /// 
    /// Should have implemented a true unicode rot13, but I was lazy
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        for c in caps[0].chars() {
            if c.is_ascii() {
                // do rot13 for ascii alphanumeric characters
                let mut c = c as u8;
                if c.is_ascii_lowercase() {
                    c = (c - b'a' + 13) % 26 + b'a';
                } else if c.is_ascii_uppercase() {
                    c = (c - b'A' + 13) % 26 + b'A';
                }
                dst.push(unsafe { char::from_u32_unchecked(c as u32) });
            } else {
                dst.push(c);
            }
        }
    }
}

fn main() {
    // match only ascii email addresses for now
    let re =
        Regex::new(r#""([[[:ascii:]]&&[^"\s\[\]\\]]*?@[[[:ascii:]]&&[^"\s]\[\]\\]*)""#).unwrap();

    let mut line = String::new();

    while let Ok(n) = stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let result = re.replace(&line, Rot8000);
        print!("{}", result);
        line.clear();
    }
}
