use std::fs::File;
use std::io::{stdout, stdin, Write};

pub fn gen() {
print!("your name: ");
            let _ = stdout().flush();
            let mut name: String = String::new();
            let _ = stdin().read_line(&mut name);
            print!("project year: ");
            let _ = stdout().flush();
            let mut year = String::new();
            let _ = stdin().read_line(&mut year);
            println!("Generating a license...");
            let license = File::create("LICENSE");
            let txt = format!(
                r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
                year.trim(), name.trim()
            );
            let _ = license.expect("error").write_all(txt.as_bytes());       
}
