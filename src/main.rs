use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest;
use self_update::Status as UpdateStatus;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio;

#[derive(Debug, Clone)]
struct Endpoint {
    name: String,
    url: String,
}

#[derive(Debug)]
struct TestResult {
    name: String,
    url: String,
    response_time: Option<Duration>,
    error: Option<String>,
}

impl TestResult {
    fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            response_time: None,
            error: None,
        }
    }

    fn with_success(mut self, duration: Duration) -> Self {
        self.response_time = Some(duration);
        self
    }

    fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }
}

async fn test_endpoint(endpoint: &Endpoint) -> TestResult {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    // 构建JSON-RPC请求体
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTipAccounts",
        "params": []
    });

    let start = Instant::now();
    let full_url = format!("{}/api/v1/getTipAccounts", endpoint.url);
    match client
        .post(&full_url)
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await
    {
        Ok(response) => {
            let duration = start.elapsed();
            if response.status().is_success() {
                TestResult::new(endpoint.name.clone(), endpoint.url.clone()).with_success(duration)
            } else {
                TestResult::new(endpoint.name.clone(), endpoint.url.clone())
                    .with_error(format!("HTTP {}", response.status()))
            }
        }
        Err(e) => {
            TestResult::new(endpoint.name.clone(), endpoint.url.clone()).with_error(e.to_string())
        }
    }
}

async fn test_all_endpoints(endpoints: Vec<Endpoint>) -> Vec<TestResult> {
    let mut handles = Vec::new();

    for endpoint in endpoints {
        let handle = tokio::spawn(async move { test_endpoint(&endpoint).await });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("任务执行错误: {}", e),
        }
    }

    results
}

fn print_results(mut results: Vec<TestResult>) {
    // 按响应时间排序，成功的请求在前，失败的在后
    results.sort_by(|a, b| match (&a.response_time, &b.response_time) {
        (Some(a_time), Some(b_time)) => a_time.cmp(b_time),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.name.cmp(&b.name),
    });

    println!("🚀 网络速度测试结果");
    println!("{}", "=".repeat(60));

    for (index, result) in results.iter().enumerate() {
        let rank = index + 1;

        match &result.response_time {
            Some(duration) => {
                let ms = duration.as_millis();
                println!("#{} 🟢 {} - {}ms", rank, result.name, ms);
                println!("    URL: {}", result.url);
            }
            None => {
                println!("#{} 🔴 {} - 失败", rank, result.name);
                println!("    URL: {}", result.url);
                if let Some(error) = &result.error {
                    println!("    错误: {}", error);
                }
            }
        }
        println!();
    }
}

fn get_mainnet_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint {
            name: "🇯🇵 Tokyo".to_string(),
            url: "https://tokyo.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🌐 Mainnet".to_string(),
            url: "https://mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇳🇱 Amsterdam".to_string(),
            url: "https://amsterdam.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇩🇪 Frankfurt".to_string(),
            url: "https://frankfurt.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇬🇧 London".to_string(),
            url: "https://london.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇺🇸 New York".to_string(),
            url: "https://ny.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇺🇸 Salt Lake City".to_string(),
            url: "https://slc.mainnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇸🇬 Singapore".to_string(),
            url: "https://singapore.mainnet.block-engine.jito.wtf".to_string(),
        },
    ]
}

fn get_testnet_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint {
            name: "🌍 Testnet".to_string(),
            url: "https://testnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇺🇸 Dallas (Testnet)".to_string(),
            url: "https://dallas.testnet.block-engine.jito.wtf".to_string(),
        },
        Endpoint {
            name: "🇺🇸 New York (Testnet)".to_string(),
            url: "https://ny.testnet.block-engine.jito.wtf".to_string(),
        },
    ]
}

#[derive(Parser)]
#[command(
    name = "jito-speedtest",
    version=option_env!("VERGEN_GIT_DESCRIBE").unwrap_or("unknown"),
    about = "Jito 区块引擎节点连接速度测试"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 打印版本信息
    Version,

    /// 检查并更新到最新版本
    Update,

    /// 测试节点
    Run {
        /// 测试 testnet 网络节点 (默认测试 mainnet)
        #[arg(short, long)]
        testnet: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Run { testnet: false }) {
        Commands::Version => {
            println!("当前版本: {}", env!("VERGEN_GIT_DESCRIBE"));
        }

        Commands::Update => {
            let current_version = env!("VERGEN_GIT_DESCRIBE");
            let status = tokio::task::spawn_blocking(move || {
                self_update::backends::github::Update::configure()
                    .repo_owner("cfanbo")
                    .repo_name("jito-speedtest")
                    .bin_name("jito-speedtest")
                    .show_download_progress(true)
                    .current_version(&current_version.trim_start_matches('v'))
                    .build()
                    .and_then(|u| u.update())
            })
            .await??;

            match status {
                UpdateStatus::UpToDate(version) => {
                    println!("\n✅ 已是最新版本: v{}", version);
                }
                UpdateStatus::Updated(version) => {
                    println!("✅ 成功更新到版本: v{}", version);
                }
            }
        }

        Commands::Run { testnet } => {
            let endpoints = if testnet {
                println!("🧪 测试 Testnet 网络节点...");
                get_testnet_endpoints()
            } else {
                println!("🌐 测试 Mainnet 网络节点...");
                get_mainnet_endpoints()
            };

            println!("开始测试网络连接速度,请稍候...\n");

            let results = test_all_endpoints(endpoints).await;
            print_results(results);
        }
    }
    Ok(())
}
