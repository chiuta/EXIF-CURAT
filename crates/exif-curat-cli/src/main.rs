use exif_curat_core::{scan, strip};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("exif-curat {}", env!("CARGO_PKG_VERSION")),
        ["scan", path] => {
            let data = read(path);
            match scan(&data) {
                Ok(found) if found.is_empty() => println!("{path}: no metadata segments"),
                Ok(found) => {
                    for (seg, kind) in found {
                        println!(
                            "{path}: {kind:?} at offset {} ({} bytes)",
                            seg.offset, seg.len
                        );
                    }
                }
                Err(e) => fail(&format!("{path}: {e:?}")),
            }
        }
        ["strip", input, output] => match strip(&read(input)) {
            Ok(out) => {
                if let Err(e) = std::fs::write(output, out) {
                    fail(&format!("{output}: {e}"));
                }
            }
            Err(e) => fail(&format!("{input}: {e:?}")),
        },
        _ => {
            eprintln!("usage: exif-curat scan FILE | strip IN OUT | --version");
            std::process::exit(2);
        }
    }
}

fn read(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| {
        fail(&format!("{path}: {e}"));
        Vec::new()
    })
}

fn fail(msg: &str) {
    eprintln!("{msg}");
    std::process::exit(1);
}
