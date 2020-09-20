use clap::Clap;
mod runtime;

#[derive(Clap, Debug)]
struct Opt {
    #[clap(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Clap, Debug)]
enum SubCmd {
    #[clap(name = "meta")]
    Meta(Meta)
}

#[derive(Clap, Debug)]
struct Meta {
    path: std::path::PathBuf,
}


fn main() -> Result<(), std::io::Error> {
    let opt: Opt = Opt::parse();
    println!("{:?}", opt);

    match opt.sub_cmd {
        SubCmd::Meta(meta) => {
            let wasm = runtime::load(meta.path)?;
            match runtime::run(&wasm) {
                Ok(_) => println!("it worked"),
                Err(err) => eprintln!("{}", err),
            }
        }
    }
    Ok(())
}
