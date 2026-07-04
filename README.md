# t — 命令行单词翻译工具

基于 Google 翻译接口的命令行翻译工具，支持中英互译和管道输入。

## 安装

```bash
# 从源码编译安装
git clone git@github.com:alexniver/tr.git
cd t
cargo install --path .
```

## 用法

```bash
# 中译英
t 你好
# → Hello

# 英译中
t Hello
# → 你好

# 管道输入
echo "世界" | t
# → world

# 指定目标语言（法语）
t Hello -t fr
# → Bonjour

# 指定源语言
t Hello -s en -t zh-CN
```

## 选项

| 参数 | 说明 |
|------|------|
| `TEXT` | 待翻译文本，不填则从标准输入读取 |
| `-t, --target` | 目标语言，默认根据首字符自动检测 |
| `-s, --source` | 源语言，默认自动检测 |
| `-h, --help` | 显示帮助信息 |

## 自动语言检测

- 首字符为英文字母 → 目标语言为中文（`zh-CN`）
- 首字符为其他字符 → 目标语言为英文（`en`）

手动指定 `-t` 可覆盖此行为。

## 依赖

- [clap](https://crates.io/crates/clap) — 命令行参数解析
- [reqwest](https://crates.io/crates/reqwest) — HTTP 请求
- [serde_json](https://crates.io/crates/serde_json) — JSON 解析
- [urlencoding](https://crates.io/crates/urlencoding) — URL 编码
