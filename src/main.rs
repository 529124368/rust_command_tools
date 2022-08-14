use clap::{arg, Command};
use colored::*;
use rust_embed::RustEmbed;
use std::{
    ffi::OsStr,
    io::{Read, Write},
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
                .about("使用godot-tust默认模版创建一个工程")
                .arg(arg!(<PROJECT_NAME> "project name"))
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("class")
                .about("快速创建godot-tust的rust GDnative脚本")
                .arg(arg!(<CLASS_NAME> "class name"))
                .arg(arg!([NODE_NAME] "node name"))
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
        //
        Some(("add_commit_push", sub_matches)) => {
            if send_cmd("git pull origin master") == ExitStatus::from_raw(0) {
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
        //
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
        //
        Some(("new", sub_matches)) => {
            let mes = sub_matches
                .get_one::<String>("PROJECT_NAME")
                .expect("please input project name");
            //export
            let root = "./".to_string() + mes + "/";
            file_tool::create_dir(&root).unwrap();
            for f in Asset::iter() {
                if !f.eq("template.txt") {
                    let mut handle = file_tool::super_create(&(root.to_string() + f.as_ref()));
                    let s = String::from_utf8_lossy(Asset::get(f.as_ref()).unwrap().data.as_ref())
                        .replace("@_", mes);
                    handle.write_all(s.as_bytes()).unwrap();
                    println!("{}", ("created: ".to_string() + f.as_ref()));
                }
            }
            println!(
                "{}",
                ("SUCCESS: A new project has been created ".to_string() + mes).green()
            );
        }
        //
        Some(("class", sub_matches)) => {
            let class_name = sub_matches
                .get_one::<String>("CLASS_NAME")
                .expect("please input class name");
            //修改lib.rs 文件追加class
            if let Ok(mut f) = file_tool::read_file("./rust/src/lib.rs") {
                //替换文字
                let mut fbuf = String::new();
                f.read_to_string(&mut fbuf).unwrap();
                let ss = fbuf.replace(
                    "fn init(handle: InitHandle) {",
                    &("fn init(handle: InitHandle) {\n    handle.add_class::<".to_string()
                        + class_name
                        + "::"
                        + &class_name[..1].to_uppercase()
                        + &class_name[1..]
                        + ">();"),
                );
                let ss = ("mod ".to_string() + class_name + ";\n").to_string() + &ss;
                //读取
                let mut f = file_tool::over_write_open("./rust/src/lib.rs").unwrap();
                f.write_all(ss.as_bytes()).unwrap();
                f.flush().unwrap();
                //添加对应的rust脚本文件
                if let Some(node_name) = sub_matches.get_one::<String>("NODE_NAME") {
                    //自定义场合
                    let f =
                        String::from_utf8_lossy(Asset::get("template.txt").unwrap().data.as_ref())
                            .replace("_@", node_name)
                            .replace(
                                "@_",
                                &(class_name[..1].to_uppercase().to_string() + &class_name[1..]),
                            );

                    let mut handle =
                        file_tool::create_file(&("./rust/src/".to_string() + class_name + ".rs"))
                            .unwrap();
                    handle.write_all(f.as_bytes()).unwrap();
                } else {
                    //非自定义场合 node
                    //自定义场合
                    let f =
                        String::from_utf8_lossy(Asset::get("template.txt").unwrap().data.as_ref())
                            .replace("_@", "Node")
                            .replace(
                                "@_",
                                &(class_name[..1].to_uppercase().to_string() + &class_name[1..]),
                            );

                    let mut handle =
                        file_tool::create_file(&("./rust/src/".to_string() + class_name + ".rs"))
                            .unwrap();
                    handle.write_all(f.as_bytes()).unwrap();
                }
            } else {
                println!("{}", "can't find file :  lib.rs".red());
            }
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
