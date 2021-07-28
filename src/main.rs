use heck::KebabCase;
use std::collections::HashSet;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The input string to slugify.
    input: String,
}

fn main() {
    let chs_to_remove = vec!['\''].into_iter().collect::<HashSet<_>>();
    let args = Cli::from_args();
    print!(
        "{}",
        args.input
            .chars()
            .filter(|ch| !chs_to_remove.contains(ch))
            .collect::<String>()
            .to_kebab_case()
    );
}
