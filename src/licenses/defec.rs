use std::fs::File;
use std::io::Write;

pub fn gen() {
    println!("Generating a license...");
    let license = File::create("LICENSE");
    let txt = format!(
        "DEFEC LISENCE v1.0
This license is written in English.
You have these rights.
- Using data applied this license
- Modificating data applied this license
- Distributing data applied this license
You also can use data applied this license even if you want to use for commercial purposes
And, you don't have to descript author(s) of data applied this license.
However, you are guaranteed nothing by author(s) of data applied this license.
If you want to change the license, you must change at least one byte data."
    );
    let _ = license.expect("error").write_all(txt.as_bytes());
}
