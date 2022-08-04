use clap::{arg, Command};
use colored::*;
use rust_embed::RustEmbed;
use std::{
    ffi::OsStr,
    io::Write,
    os::windows::process::ExitStatusExt,
    process::{Command as Cmd, ExitStatus},
};

mod tools;
use tools::file_tool;
#[derive(RustEmbed)]
#[folder = "godotTemplate"]
struct Asset;

fn main() {
    let comm = Command::new("Git + Godot Tools")
        .about("Git提交快捷命令行+ Godot游戏引擎之rust环境模版快速创建")
        .subcommand(
            Command::new("add_commit")
                .about("本地添加和提交到本地仓库的快捷指令")
                .short_flag('c')
                .arg(arg!(-w <COMMENT> "注释"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add_commit_push")
                .about("本地添加和提交到本地仓库并且push到远端的快捷指令")
                .short_flag('C')
                .arg(arg!(-w <COMMENT> "注释"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("register_add_commit_push")
                .about("新规的场景使用,从建立和远端的链接到本地添加和提交到本地仓库并且push到远端的快捷指令")
                .short_flag('r')
                .arg(arg!(-l <REMOTE_URL> "远程地址"))
                .arg(arg!(-w <COMMENT> "注释"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("new")
                .about("godot游戏引擎的rust脚本模版创建")
                .arg(arg!(<PROJECT_NAME> "工程名字"))
                .arg_required_else_help(true),
        );
    //dispact
    match comm.get_matches().subcommand() {
        Some(("add_commit", sub_matches)) => {
            if send_cmd("git add .") == ExitStatus::from_raw(0) {
                let mes = sub_matches
                    .get_one::<String>("COMMENT")
                    .expect("please input comment");
                let mes = "git commit -m ".to_string() + mes;
                send_cmd(mes);
            }
        }
        Some(("add_commit_push", sub_matches)) => {
            if send_cmd("git add .") == ExitStatus::from_raw(0) {
                let mes = sub_matches
                    .get_one::<String>("COMMENT")
                    .expect("please input comment");
                let mes = "git commit -m ".to_string() + mes;
                if send_cmd(mes) == ExitStatus::from_raw(0) {
                    send_cmd("git push origin master");
                }
            }
        }
        Some(("register_add_commit_push", sub_matches)) => {
            let mes = sub_matches
                .get_one::<String>("REMOTE_URL")
                .expect("please input url");
            let mes = "git remote add origin ".to_string() + mes;
            if send_cmd(mes) == ExitStatus::from_raw(0) {
                if send_cmd("git add .") == ExitStatus::from_raw(0) {
                    let mes = sub_matches
                        .get_one::<String>("COMMENT")
                        .expect("please input comment");
                    let mes = "git commit -m ".to_string() + mes;
                    if send_cmd(mes) == ExitStatus::from_raw(0) {
                        send_cmd("git push origin master");
                    }
                }
            }
        }
        Some(("new", sub_matches)) => {
            let mes = sub_matches
                .get_one::<String>("PROJECT_NAME")
                .expect("please input project name");
            //export
            let root = "./".to_string() + mes + "/";
            file_tool::create_dir(&root).unwrap();
            for f in Asset::iter() {
                let mut handle = file_tool::super_create(&(root.to_string() + f.as_ref()));
                let s = String::from_utf8_lossy(Asset::get(f.as_ref()).unwrap().data.as_ref())
                    .replace("@_", mes);
                handle.write_all(s.as_bytes()).unwrap();
                println!("{}", ("created: ".to_string() + f.as_ref()));
            }
            println!(
                "{}",
                ("SUCCESS: A new project has been created ".to_string() + mes).green()
            );
        }
        _ => unreachable!(),
    }
}

fn send_cmd(str: impl AsRef<OsStr>) -> ExitStatus {
    let mut command = Cmd::new("cmd");
    let res = command
        .arg("/C")
        .arg(str)
        .output()
        .expect("parse command has error");
    let res_de = String::from_utf8_lossy(&res.stdout);
    if !res_de.is_empty() {
        println!("执行结果:{}", res_de);
    }
    res.status
}
