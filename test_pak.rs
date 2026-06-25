use std::collections::HashMap;
mod framework;
fn main() {
    let mut pak = framework::paklib::PakInterface::new();
    pak.set_resource_folder(".");
    if pak.add_pak_file("main.pak") {
        let files = pak.list_files();
        let mut v: Vec<&String> = files.iter().filter(|s| {
            let l = s.to_lowercase();
            l.contains("popcap") || l.contains("partner") || l.contains("pvz_logo") || l.contains("titlescreen") || l.contains("loadbar") || l.contains("sodroll") || l.starts_with("_")
        }).collect();
        v.sort();
        for f in v {
            println!("{}", f);
        }
    }
}