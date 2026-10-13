use clap::{Parser, Subcommand, error::ErrorKind};
use dependencies_cli::{Direction, Failure, Operation, Workspace};
use serde_json::{Value, json};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "由 LLM 维护的工作目录依赖管理工具", color = clap::ColorChoice::Never)]
#[command(
    after_help = "A 依赖 B：add A B；B 变化后：dependents B。路径相对于 --root。\n除 help/version 外均向 stdout 输出 JSON，成功退出 0，失败退出 2。\n批量操作是一个事务，失败全部回滚；查询缺失文件时 data 仍保留已有依赖。"
)]
struct Cli {
    /// 工作目录；默认当前目录，不自动向上寻找数据库
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 只读列出 Git 管理范围及范围外的旧对象和关系
    Scope,
    /// 创建本地数据库；已初始化则不重复创建
    Init,
    /// 登记 A 依赖 B；同关系同原因重复调用为无操作
    Add {
        dependent: String,
        dependency: String,
        #[arg(long)]
        reason: Option<String>,
    },
    /// 删除关系；允许文件已经缺失，关系不存在则无操作
    Remove {
        dependent: String,
        dependency: String,
        /// 仅在现有原因仍与预览相同时删除，供批量清理防止覆盖并发修改
        #[arg(long)]
        expected_reason: Option<String>,
    },
    /// 显式清理已不被 Git 跟踪且没有任何关系的旧对象
    Forget { file: String },
    /// 修改关系端点或原因；不会覆盖其他已有关系
    Update {
        dependent: String,
        dependency: String,
        #[arg(long)]
        new_dependent: Option<String>,
        #[arg(long)]
        new_dependency: Option<String>,
        #[arg(long, conflicts_with = "clear_reason")]
        reason: Option<String>,
        #[arg(long)]
        clear_reason: bool,
    },
    /// 从 JSON 操作数组执行一个事务；文件路径相对于调用目录，- 表示 stdin
    Batch {
        #[arg(default_value = "-")]
        input: String,
    },
    /// 查询该文件依赖谁；--transitive 展开间接关系
    Dependencies {
        file: String,
        #[arg(long)]
        transitive: bool,
    },
    /// 查询谁依赖该文件；--transitive 展开间接关系
    Dependents {
        file: String,
        #[arg(long)]
        transitive: bool,
    },
}

fn run(cli: Cli) -> Result<Value, Failure> {
    let workspace = Workspace::new(&cli.root)?;
    let operation = match cli.command {
        Command::Scope => return workspace.scope(),
        Command::Init => return workspace.init(),
        Command::Dependencies { file, transitive } => {
            return workspace.query(&file, Direction::Dependencies, transitive);
        }
        Command::Dependents { file, transitive } => {
            return workspace.query(&file, Direction::Dependents, transitive);
        }
        Command::Batch { input } => {
            let mut content = String::new();
            if input == "-" {
                io::stdin()
                    .read_to_string(&mut content)
                    .map_err(|e| Failure::new("IO_ERROR", e.to_string()))?;
            } else {
                content = std::fs::read_to_string(input)
                    .map_err(|e| Failure::new("IO_ERROR", e.to_string()))?;
            }
            let operations: Vec<Operation> = serde_json::from_str(&content)
                .map_err(|e| Failure::new("INVALID_BATCH", e.to_string()))?;
            return workspace.apply(&operations);
        }
        Command::Add {
            dependent,
            dependency,
            reason,
        } => Operation::Add {
            dependent,
            dependency,
            reason,
        },
        Command::Remove {
            dependent,
            dependency,
            expected_reason,
        } => Operation::Remove {
            dependent,
            dependency,
            expected_reason,
        },
        Command::Forget { file } => Operation::Forget { file },
        Command::Update {
            dependent,
            dependency,
            new_dependent,
            new_dependency,
            reason,
            clear_reason,
        } => Operation::Update {
            dependent,
            dependency,
            new_dependent,
            new_dependency,
            reason,
            clear_reason,
        },
    };
    workspace.apply(&[operation])
}

fn main() {
    let result = match Cli::try_parse() {
        Ok(cli) => run(cli),
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            let _ = error.print();
            return;
        }
        Err(error) => Err(Failure::new("INVALID_ARGUMENT", error.to_string())),
    };
    let (response, exit) = match result {
        Ok(data) => (json!({"ok": true, "data": data, "error": null}), 0),
        Err(error) => (json!({"ok": false, "data": error.data, "error": error}), 2),
    };
    let mut stdout = io::stdout().lock();
    if serde_json::to_writer_pretty(&mut stdout, &response).is_err() || writeln!(stdout).is_err() {
        std::process::exit(2);
    }
    std::process::exit(exit);
}
