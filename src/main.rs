use clap::{clap_app, crate_version};

fn main() {
    let clap = clap_app!(mdrend => 
                     (version:crate_version!())
                     (author: "Jacky.yao")
                     (about: "Renders markdown as you like")
     )
    .get_matches();
    println!("done");
}
