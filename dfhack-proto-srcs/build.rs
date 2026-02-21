use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

struct Cp437Field {
    file: &'static str,
    message: &'static str,
    field_name: &'static str,
    field_tag: u32,
}

const CP437_FIELDS: &[Cp437Field] = &[
    Cp437Field {
        file: "RemoteFortressReader.proto",
        message: "MaterialDefinition",
        field_name: "name",
        field_tag: 3,
    },
    Cp437Field {
        file: "RemoteFortressReader.proto",
        message: "WorldMap",
        field_name: "name",
        field_tag: 3,
    },
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DFHACK_ZIP_URL");
    println!("cargo:rerun-if-env-changed=DFHACK_DOWNLOAD");

    let regen = match std::env::var("DFHACK_DOWNLOAD") {
        Ok(val) => val == "1",
        Err(_) => false,
    };

    if !regen {
        return;
    }

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut proto_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    proto_dir.push("src");
    proto_dir.push("protos");

    if proto_dir.exists() {
        std::fs::remove_dir_all(&proto_dir).unwrap();
    }
    std::fs::create_dir_all(&proto_dir).unwrap();

    // Download the file
    let dfhack_archive_path = Path::new(&out_dir).join("dfhack.zip");
    let mut dfhack_archive = File::create(&dfhack_archive_path).unwrap();
    let dfhack_url = match std::env::var("DFHACK_ZIP_URL") {
        Ok(val) => val,
        Err(_) => "https://github.com/DFHack/dfhack/archive/refs/tags/53.10-r1.zip".to_string(),
    };
    let mut dfhack_download_request = reqwest::blocking::get(dfhack_url).unwrap();
    std::io::copy(&mut dfhack_download_request, &mut dfhack_archive).unwrap();

    // Extract the protos
    let mut protos = Vec::new();
    let dfhack_archive = File::open(&dfhack_archive_path).unwrap();
    let mut dfhack_archive = zip::ZipArchive::new(dfhack_archive).unwrap();
    for i in 0..dfhack_archive.len() {
        let mut file = dfhack_archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if outpath.extension().unwrap_or_default() != "proto" {
            continue;
        }

        let mut dest = proto_dir.clone();
        dest.push(PathBuf::from(outpath.file_name().unwrap()));

        let mut outfile = File::create(&dest).unwrap();
        std::io::copy(&mut file, &mut outfile).unwrap();
        protos.push(dest);
    }

    patch_cp437_fields(&proto_dir);
}

/// Patch proto fields that carry CP437-encoded text from `string` to
/// `bytes` so prost generates `Vec<u8>` instead of `String`.
fn patch_cp437_fields(proto_dir: &Path) {
    let files: Vec<&str> = CP437_FIELDS
        .iter()
        .map(|f| f.file)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    for filename in files {
        let path = proto_dir.join(filename);
        let fields: Vec<&Cp437Field> = CP437_FIELDS
            .iter()
            .filter(|f| f.file == filename)
            .collect();

        let reader = BufReader::new(File::open(&path).unwrap());
        let mut output = Vec::new();
        let mut current_message: Option<String> = None;
        let mut brace_depth: u32 = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let trimmed = line.trim();

            // Track message context
            if let Some(rest) = trimmed.strip_prefix("message ") {
                let msg_name = rest
                    .trim_end_matches('{')
                    .trim();
                current_message = Some(msg_name.to_string());
                brace_depth = 0;
            }

            for ch in trimmed.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        brace_depth = brace_depth.saturating_sub(1);
                        if brace_depth == 0 {
                            current_message = None;
                        }
                    }
                    _ => {}
                }
            }

            let patched = match &current_message {
                Some(msg) => {
                    let matching = fields
                        .iter()
                        .find(|f| f.message == msg);
                    match matching {
                        Some(field) => {
                            let target = format!(
                                "string {} = {};",
                                field.field_name, field.field_tag
                            );
                            let replacement = format!(
                                "bytes {} = {};",
                                field.field_name, field.field_tag
                            );
                            line.replace(&target, &replacement)
                        }
                        None => line.clone(),
                    }
                }
                None => line.clone(),
            };

            writeln!(output, "{}", patched).unwrap();
        }

        std::fs::write(&path, output).unwrap();
    }
}
