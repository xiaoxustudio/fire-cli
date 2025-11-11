# fire

一个基于`rust`开发的终端工具，用于快速操作一系列操作。

## 使用

```bash
> fire-cli --help

一个简单的实用工具

Usage: fire-cli.exe <COMMAND>

Commands:
  file  文件相关命令
  link  链接相关命令
  open  打开相关命令
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

# file

```bash
> fire-cli file --help

文件相关命令

Usage: fire-cli.exe file <COMMAND>

Commands:
  list    列出文件/夹目录
  open    打开文件
  delete  删除文件/目录
  rename  重命名文件/目录
  write   写入文件（连续两次回车结束写入）
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# link

```bash
> fire-cli link --help

链接相关命令

Usage: fire-cli.exe link <COMMAND>

Commands:
  create  创建链接
  remove  移除链接
  show    显示链接
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# open

```bash
> fire-cli open --help

打开相关命令

Usage: fire-cli.exe open <TARGET>

Arguments:
  <TARGET>

Options:
  -h, --help  Print help
```

# 关于

作者：[徐然](https://github.com/xiaoxustudio)  

联系方式：[xugame@qq.com](emailto://xugame@qq.com)

欢迎提出您宝贵的 **issue**，我们将会处理。

# LICENSE

[MIT](./LICENSE)