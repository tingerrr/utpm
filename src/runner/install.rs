use std::{collections::VecDeque, os::unix::fs::symlink};

use crate::parser::CLIOptions;
use crate::utils::{Config, ListDependencies, Dependency};
use crate::utils::paths::{get_current_utpm, get_current_config, get_global_utpm, check_existing_symlink};
use crate::utils::state::{GoodState, ErrorState};

use super::{check_help, CommandUTPM};


pub struct Install {
    options: VecDeque<CLIOptions>,
}

impl CommandUTPM for Install {
    fn new(options: VecDeque<CLIOptions>) -> Self {
        Self { options }
    }

    fn run(&mut self) -> Result<GoodState, ErrorState> {
        if check_help(&self.options) {
            Self::help();
            return Ok(GoodState::Help);
        }

        let current_directory = match get_current_utpm() {
            Ok(val) => val,
            Err(_) => {
                return Err(ErrorState::CurrentDirectoryError(String::from(
                    "There is no \".utpm\" folder in your directory.",
                )))
            }
        };

        let token = match self.options.pop_front() {
            Some(val) => match val {
                CLIOptions::Token(string) => string,
                _ => {
                    return Err(ErrorState::UnexpectedTokenError(String::from(
                        "there is no token containing the name/link of the project",
                    )))
                }
            },
            None => return Ok(GoodState::Help),
        };

        let current_config = get_current_config()?;

        let mut conf = Config::load(&current_config);
        let mut global_conf = ListDependencies::load();
        let dependency = Dependency::from_link(&token);
        let name = dependency.name.clone();
        let sym = current_directory.clone() + "/" + name.as_str();

        if !global_conf.dependencies.contains(&dependency) {
            match global_conf.add(&token) {
                Ok(val) => match val {
                    GoodState::Good(string) => println!("{string}"),
                    GoodState::NothingToDo(string) => println!("{string}"),
                    _ => (),
                },
                Err(err) => return Err(err),
            };
        }

        //TODO: Faire un fichier contenant tous les projets déjà existant sur "awesome-typst" + discord/#showcase
        //TODO: Faire un lecteur de config .utpm/.package → fallback sur la création provenant du github.com → Voir en dessous
        //TODO: Faire une recherche du "main" pour les projets n'ayant pas de .utpm
        //TODO: Commande create doit créer le dossier .utpm avec un fichier ".package" contenant toute la config de son projet → NON, utiliser config
        //TODO: Ranger. Genre runner.rs trop de conneries
        //TODO: Finir les ifs un peu partout + better text

        let globpath: String = get_global_utpm();

        if !conf.dependencies.contains(&dependency) {
            conf.dependencies.push(dependency);
            conf.write(&current_config);
        }

        if check_existing_symlink(&sym) {
            return Ok(GoodState::NothingToDo(String::from(
                "Symlink already exist",
            )));
        }

        match symlink(
            globpath + "/" + name.as_str(),
            current_directory.clone() + "/" + name.as_str(),
        ) {
            Ok(_) => Ok(GoodState::Good("good".to_string())),
            Err(val) => Err(ErrorState::SymlinkUnixError(val.to_string())),
        }
    }

    fn help() {
        println!("Unofficial Typst Package Manager (utpm).");
        println!();
        println!("Usage:");
        println!("  utpm install <url|name>");
        println!();
        println!("Description:");
        println!("  This command install a package from an URL or a NAME which refers to");
        println!("  the file at github.com/ThumusLive/unofficial-typst-package-manager/list_projects.json");
        println!("  It add the dependency in the current and globaly utpm config file.");
        println!();
        println!("Options: ");
        println!("  --help, -h, h                           Print this message");
    }

}