use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("include_all_assets.rs");

    let mut file = File::create(dest_path).unwrap();
    file.write_all("pub fn include_all_assets(in_memory: &mut crate::InMemoryAssetIo){\n".as_ref())
        .unwrap();

    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets");
    visit_dirs(&dir)
        .iter()
        .filter(|path| {
            ["png", "ttf"].contains(&path.extension().and_then(|e| e.to_str()).unwrap_or("zut"))
        })
        .map(|path| (path, path.strip_prefix(&dir).unwrap()))
        .for_each(|(fullpath, path)| {
            file.write_all(
                format!(
                    r#"in_memory.add_entity(std::path::Path::new({:?}), include_bytes!({:?}));
"#,
                    path.to_string_lossy(),
                    fullpath.to_string_lossy()
                )
                .as_ref(),
            )
            .unwrap();
        });

    file.write_all("}".as_ref()).unwrap();

    println!("cargo:rerun-if-changed=include_all_assets.rs");
}

fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut collected = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collected.append(&mut visit_dirs(&path))
            } else {
                collected.push(path)
            }
        }
    }
    collected
}
