use clap::Parser;
use reqwest::Client;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "tr", about = "命令行单词翻译工具")]
pub struct Args {
    /// 待翻译的文本，不填则从标准输入读取
    #[arg(value_name = "TEXT", default_value = "-")]
    pub text: String,

    /// 目标语言，默认英文（en）或中文（zh-CN）
    #[arg(short = 't', long, default_value = "en")]
    pub target: String,

    /// 源语言，默认自动检测
    #[arg(short = 's', long, default_value = "auto")]
    pub source: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let text = if args.text != "-" {
        args.text
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf.trim().to_string()
    };

    if text.is_empty() {
        eprintln!("错误：没有输入要翻译的内容");
        std::process::exit(1);
    }

    let client = Client::new();
    let result = translate(&client, &text, &args.target, &args.source).await?;
    println!("{}", result);
    Ok(())
}

async fn translate(
    client: &Client,
    text: &str,
    target: &str,
    source: &str,
) -> anyhow::Result<String> {
    let encoded = urlencoding::encode(text);
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        source, target, encoded
    );

    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        anyhow::bail!("翻译服务返回状态码 {}: {}", status, body);
    }

    let json: serde_json::Value = serde_json::from_str(&body)?;
    let translated = json[0][0][0]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "[翻译失败]".to_string());

    Ok(translated)
}
