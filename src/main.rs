use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "runnr", about = "An example command line application.")]
enum Runnr {
    #[structopt(about = "Creates a new item.")]
    New {
        #[structopt(long)]
        option: Option<String>,
    },
    #[structopt(about = "Executes an item.")]
    Exec {
        #[structopt(long)]
        option: Option<String>,
    },
}

fn main() {
    let opt = Runnr::from_args();

    match opt {
        Runnr::New { option } => {
            println!("New command called.");
            if let Some(o) = option {
                println!("Option: {}", o);
            }
        }
        Runnr::Exec { option } => {
            println!("Exec command called.");
            if let Some(o) = option {
                println!("Option: {}", o);
            }
        }
    }
}
