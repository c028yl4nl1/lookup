// Pegar os argumentos do Usuario

use colored::*;
use std::collections::HashMap;
use std::env::args;
use std::fmt::format;
use std::io::Write;
use std::net::*;
use std::process::exit;
use std::{
    path::{PathBuf, StripPrefixError},
    str::FromStr,
};
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;
#[derive(Debug, Clone)]
struct arg_user {
    target: String,
    file_path: PathBuf,
}

fn recev_args(target: String, file: String) -> Option<arg_user> {
    let mut path_0 = PathBuf::new();
    let file_path = PathBuf::from_str(file.as_str());
    match file_path {
        Ok(Path) => {
            if Path.exists() {
                path_0.push(Path);
            } else {
                eprintln!("PathFile not exist ");
                std::process::exit(1);
            }
        }
        Err(l) => {
            eprintln!("PathFile not exist ");
            std::process::exit(1);
        }
    }

    Some(arg_user {
        file_path: path_0,
        target: target,
    })
}

fn main() {
    let recev_args_: Vec<String> = args().collect();
    if recev_args_.len() < 3 {
        eprintln!("/bin target filename");
        std::process::exit(1);
    }
    let recev = recev_args(recev_args_[1].to_owned(), recev_args_[2].to_owned());
    let mut subdmains_hash: HashMap<String, Vec<IpAddr>> = HashMap::new();
    match recev {
        Some(all_info) => {
            let path = all_info.file_path;
            let host = all_info.target;
            // Vereficar se o host existe
            if resolver(&host).is_none() {
                eprintln!("{}", "Host Not existe".bright_red());
                exit(1);
            }
            for subdominios in std::fs::read_to_string(path).unwrap().lines() {
                let dns_string = format!("{}.{}", subdominios, &host);
                match resolver(&dns_string) {
                    Some(ipvAddrip4) => {
                        println!(
                            "Found ->:  {}",
                            format!("{}\r\r", dns_string).bright_green()
                        );
                        subdmains_hash.insert(dns_string.to_owned(), ipvAddrip4);
                    }
                    None => {
                        eprintln!("Try error -> {}", format!("{}", dns_string).bright_red());

                        continue;
                    }
                }
            }
        }
        None => {}
    }
    #[cfg(target_os = "windows")]
    std::process::Command::new("cls").status();
    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status();

    animation();
    println!("\n");
    for (host, ips) in subdmains_hash {
        println!("Host: {} / Ips: {}",
            host.bright_green(),
            format!("{:?}", ips).bright_cyan()
        );
    }
}
fn resolver<T: AsRef<str>>(host: &T) -> Option<Vec<IpAddr>> {
    let mut resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default());
    if resolver.is_err() {
        eprintln!("{}", "Error in Build".on_bright_red());
        return None;
    }

    let va = resolver.unwrap().lookup_ip(host.as_ref());

    if let Ok(ip_lookpack) = va {
        Some(ip_lookpack.iter().collect::<Vec<IpAddr>>())
    } else {
        None
    }
}
use rand::{thread_rng, Rng};
fn animation() {
    let ascci_art = r#"

         .__                    __
  __| _/ ____    ______ |  |    ____    ____  |  | __ __ __ ______
 / __ | /    \  /  ___/ |  |   /  _ \  /  _ \ |  |/ /|  |  \\____ \
/ /_/ ||   |  \ \___ \  |  |__(  <_> )(  <_> )|    < |  |  /|  |_> >
\____ ||___|  //____  > |____/ \____/  \____/ |__|_ \|____/ |   __/
     \/     \/      \/                             \/       |__|  "#;

    for a in ascci_art.chars() {
        let number = thread_rng().gen_range(0..13);
        match number {
            1 => {
                print!("{}", a.to_string());
            }
            2 => {
                print!("{}", a.to_string().bold().green());
            }
            3 => {
                print!("{}", a.to_string().bold().black());
            }
            4 => {
                print!("{}", a.to_string().bright_black());
            }
            5 => {
                print!("{}", a.to_string().bright_blue());
            }
            6 => {
                print!("{}", a.to_string().bright_cyan());
            }
            7 => {
                print!("{}", a.to_string().bright_green());
            }
            8 => {
                print!("{}", a.to_string().bright_magenta());
            }
            9 => {
                print!("{}", a.to_string().bright_purple());
            }
            10 => {
                print!("{}", a.to_string().bright_red());
            }
            11 => {
                print!("{}", a.to_string().bright_yellow());
            }
            _ => {
                print!("{}", a.to_string().bright_white());
            }
        }
    }
}
