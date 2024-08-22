use goldenfile::Mint;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use vimcats::{vimdoc::VimDoc, FromLuaCATS, Settings, VimCats};

#[cfg(test)]
macro_rules! vimcat {
    ($($src: expr),*) => {{
        let mut vimcats = VimCats::default();
        let s = Settings::default();
        $(
            vimcats.for_help($src, &s).unwrap();
        )*
        VimDoc::from_emmy(&vimcats, &s).to_string()
    }};
}

fn run_golden(basename: String) {
    let mut mint = Mint::new("tests/golden");
    let mut goldenfile = mint.new_goldenfile(format!("{}.txt", basename)).unwrap();
    let content =
        fs::read_to_string(PathBuf::from(format!("tests/golden/{}.lua", basename))).unwrap();
    writeln!(goldenfile, "{}", vimcat!(content.as_str())).unwrap();
}

#[test]
fn golden() {
    fs::read_dir("tests/golden")
        .unwrap()
        .filter_map(|entry| entry.ok()) // Filter out errors
        .filter_map(|entry| {
            let path = entry.path(); // Store PathBuf in a variable
            if path.extension().and_then(|ext| ext.to_str()) == Some("lua") {
                path.file_stem()
                    .and_then(|stem| stem.to_str().map(|s| s.to_owned())) // Clone the basename as a String
            } else {
                None
            }
        })
        .for_each(|basename| run_golden(basename));
}
