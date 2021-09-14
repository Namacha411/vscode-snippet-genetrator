mod get_files;
mod make_snippet;

use get_files::VisitDir;
use make_snippet::Snippet;

use jsonxf::pretty_print;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let extention = opt.expansion;

    let paths = match VisitDir::new(".") {
        Ok(dir) => dir.collect::<Vec<_>>(),
        Err(why) => panic!("{:?}", why),
    };
    let paths = paths
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|e| e.extension().is_some())
        .filter(|e| e.extension().unwrap().to_str().unwrap() == &extention)
        .collect::<Vec<_>>();

    let mut snippets = String::new();
    snippets += "{";
    for path in paths {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut snippet = Snippet::new(path);
        let snippet = snippet.make_json();
        snippets += &format!("\"{}\":{},", name, snippet);
    }
    snippets += "}";
    let snippets = pretty_print(&snippets).unwrap();

    println!("{}", snippets);
}

#[derive(StructOpt)]
struct Opt {
    expansion: String
}