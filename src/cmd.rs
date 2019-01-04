extern crate colored;
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "dmenv", about = "The stupid virtualenv manager")]
pub struct Command {
    #[structopt(long = "python", help = "python binary")]
    pub python_binary: Option<String>,

    #[structopt(long = "cwd", help = "path to use as the workining directory")]
    pub working_dir: Option<String>,

    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(StructOpt)]
pub enum SubCommand {
    #[structopt(name = "clean", about = "clean existing virtualenv")]
    Clean {},

    #[structopt(name = "develop", about = "run setup.py develop")]
    Develop {},

    #[structopt(name = "install", about = "Install all dependencies")]
    Install {
        #[structopt(long = "--no-develop", help = "do not run setup.py develop")]
        no_develop: bool,

        #[structopt(long = "--no-upgrade-pip", help = "do not upgrade pip")]
        no_upgrade_pip: bool,
    },

    #[structopt(name = "bump-in-lock", about = "Bump a dependency in the lock file")]
    BumpInLock {
        #[structopt(help = "name")]
        name: String,

        #[structopt(long = "--git")]
        git: bool,

        #[structopt(help = "version")]
        version: String,
    },

    #[structopt(name = "init", about = "Initialize a new project")]
    Init {
        #[structopt(help = "Project name")]
        name: String,

        #[structopt(long = "version", help = "Project version", default_value = "0.1.0")]
        version: String,

        #[structopt(long = "author", help = "author")]
        author: Option<String>,
    },

    #[structopt(name = "lock", about = "(Re)-generate requirements.lock")]
    Lock {},

    #[structopt(name = "run", about = "Run the given binary from the virtualenv")]
    Run {
        #[structopt(name = "command")]
        cmd: Vec<String>,
    },

    #[structopt(name = "show:deps", about = "Show dependencies information")]
    ShowDeps {},

    #[structopt(name = "show:venv_path", about = "Show path of the virtualenv")]
    ShowVenvPath {},

    #[structopt(name = "upgrade-pip", about = "Upgrade pip in the virtualenv")]
    UpgradePip {},
}

pub fn print_error(description: &str) {
    eprintln!("{}: {}", "Error".bold().red(), description);
}

pub fn print_warning(description: &str) {
    eprintln!("{}: {}", "Warning".bold().yellow(), description);
}

pub fn print_info_1(message: &str) {
    println!("{} {}", "::".blue(), message);
}

pub fn print_info_2(message: &str) {
    println!("{} {}", "->".blue(), message);
}
