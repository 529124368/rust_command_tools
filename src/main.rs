use clap::{arg, Command};
use std::{
    ffi::OsStr,
    os::windows::process::ExitStatusExt,
    process::{Command as Cmd, ExitStatus},
};
fn main() {
    let comm = Command::new("Git Tools")
        .about("Git提交快捷命令行")
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
            let mes = "git remote add  origin ".to_string() + mes;
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
