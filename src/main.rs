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
            println! {"[add commit push] is success:{}",sub_matches.get_one::<String>("COMMENT").expect("please input comment")}
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
    println!("stdout:{}", res_de);
    res.status
}
