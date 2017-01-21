extern crate phf_codegen;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main(){
    let mut cmd = std::env::var_os("RUSTC")
                  .map(|c| Command::new(c))
                  .unwrap_or(Command::new("rustc"));
    let c = cmd
            .args(&["--print=target-list"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn rustc!");
    let targets = c.wait_with_output().expect("could not wait for rustc to exit");
    if !targets.status.success() {
        println!("rustc --print=target-list did not exit successfully");
        std::process::exit(1);
    }
    let output = Path::new(&::std::env::var_os("OUT_DIR").expect("OUT_DIR")).join("builtins.rs");
    let mut file = BufWriter::new(File::create(&output).expect("builtins.rs file"));

    let stdout = String::from_utf8_lossy(&targets.stdout);
    write!(&mut file, "static BUILTINS: phf::Map<&'static str, TargetInfo> = ").unwrap();
    let mut map = phf_codegen::Map::new();
    for line in stdout.lines() {
        let cfg = cfg_for_target(line);
        if !cfg.is_empty() {
            map.entry(line, &cfg);
        }
    }
    map.build(&mut file).unwrap();
    write!(&mut file, ";").unwrap();
}


fn cfg_for_target(target: &str) -> String {
    let mut cmd = std::env::var_os("RUSTC")
                  .map(|c| Command::new(c))
                  .unwrap_or(Command::new("rustc"));
    let c = cmd
            .args(&["--target", target, "--print=cfg"])
            .stdout(Stdio::piped())
            .spawn().and_then(|c| c.wait_with_output());

    if let Ok(o) = c {
        if o.status.success() {
            let string = String::from_utf8_lossy(&o.stdout);
            let (a, v, o, env, end, pwd) = parse(&string);
            format!("TargetInfo {{ \
                        arch: B({:?}), \
                        vendor: B({:?}), \
                        os: B({:?}), \
                        env: B({:?}), \
                        endian: B({:?}), \
                        pointer_width: B({:?}) \
                    }}", a, v, o, env, end, pwd)
        } else {
            println!("rustc --print=cfg --target={} did not exit successfully", target);
            String::new()
        }
    } else {
        String::new()
    }
}

fn parse(i: &str) -> (&str, &str, &str, &str, &str, &str) {
    let mut ret = ("", "", "", "", "", "");
    for line in i.lines() {
        let (c, l) = if line.starts_with("target_os") {
            (&mut ret.2, line)
        } else if line.starts_with("target_arch") {
            (&mut ret.0, line)
        } else if line.starts_with("target_vendor") {
            (&mut ret.1, line)
        } else if line.starts_with("target_env") {
            (&mut ret.3, line)
        } else if line.starts_with("target_pointer_width") {
            (&mut ret.5, line)
        } else if line.starts_with("target_endian") {
            (&mut ret.4, line)
        } else {
            continue;
        };
        *c = l.split('"').nth(1).unwrap_or("");
    }
    ret
}
