use std::collections::{HashMap, HashSet};
use std::env::args;
use humansize::{DECIMAL, format_size};
use prettytable::{row, Table, format};
use thousands::Separable;
use spinners::{Spinner, Spinners};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Please provide a directory to scan");
        std::process::exit(1);
    }

    let root = &args[1];
    if !std::path::Path::new(root).exists() {
        eprintln!("Directory does not exist");
        std::process::exit(1);
    }

    let mut count_by_ext: HashMap<String, u32> = HashMap::new();
    let mut size_by_ext: HashMap<String, u64> = HashMap::new();
    let mut exts: HashSet<String> = HashSet::new();

    let mut sp = Spinner::new(Spinners::Dots9, "Scanning files...".into());

    jwalk::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .for_each(|e| {
            let path = e.path();
            let is_dir = path.is_dir();

            if !is_dir {
                let extension = match path.extension() {
                    Some(ext) => ext.to_string_lossy().to_string(),
                    None => {
                        return;
                    }
                };

                let count = count_by_ext.entry(extension.clone()).or_insert(0);
                *count += 1;


                let metadata = match e.metadata() {
                    Ok(metadata) => metadata,
                    Err(_) => {
                        return;
                    }
                };

                let size = metadata.len();
                let total_size = size_by_ext.entry(extension.clone()).or_insert(0);
                *total_size += size;

                exts.insert(extension);
            }
        });

    sp.stop();

    let mut exts: Vec<String> = exts.into_iter().collect();
    exts.sort_by(|a, b| {
        let count_a = count_by_ext.get(a).unwrap();
        let count_b = count_by_ext.get(b).unwrap();
        count_b.cmp(count_a)
    });

    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);


    table.set_titles(row!["Extension", "Count", "Size"]);

    for ext in exts {
        let count = count_by_ext.get(&ext).unwrap();
        let size = size_by_ext.get(&ext).unwrap();
        let formatted_size = format_size(*size, DECIMAL);
        table.add_row(row![
            c -> ext,
            r -> count.separate_with_commas(),
            r -> formatted_size
        ]);
    }

    let total_count: u32 = count_by_ext.values().sum();
    let total_size: u64 = size_by_ext.values().sum();
    let formatted_total_size = format_size(total_size, DECIMAL);
    table.add_empty_row();
    table.add_row(row!["", r -> total_count.separate_with_commas(), r -> formatted_total_size]);

    println!();
    table.printstd();
}
