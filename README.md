# rust_command_tools
通过rust的clap框架，实现的一个简单的git快速提交命令行工具

使用方法
1. 下载代码 (rust开发环境 和 git工具安装)
2. cargo build --release 编译
3. 使用说明
   -  tools -c -w "add"  相当于 git add . +  git commit -m "add" 两个命令
   -  tools -C -w "add"  相当于 git add . +  git commit -m "add + git push origin master  三个命令
   -  tools -r -l http://xxxxxxxxxx.git -w "add"  相当于 git remote add origin http://xxxxxxxxxx.git + git add . +  git commit -m "add + git push origin master  `四个命令

4. godot引擎 rust脚本 环境的快速创建功能

  - tools new test_demo  创建一个godot-rust的游戏工程
  -  进入test_demo 目录
    tools class player   创建一个默认的Node节点的rust脚本，名字为Player.
    tools class player Area2D    创建一个Area2D节点的rust脚本，名字为Player.
    
  -  关于godot的节点名字，请查询godot学习网站
