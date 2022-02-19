use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    last_name: String,
    first_name: String,
}

fn main() {
    let args = Args::parse();

    let (last_name_items, first_name_items) =
        gokaku::get_all_strokes(&args.last_name[..], &args.first_name[..]);
    let result = gokaku::calc_jikaku(&last_name_items, &first_name_items);

    gokaku::print_name_info(&last_name_items, &first_name_items);
    println!("-----");
    gokaku::print_gokaku(&result);
}
