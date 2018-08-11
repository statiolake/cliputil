use std::error;
use std::fmt;
use std::result;

use isatty;

pub type Result<T> = result::Result<T, Error>;

/// 動作モード
pub enum Mode {
    /// クリップボードへ書き込み。
    Write,
    /// クリップボードの内容を表示。
    Read,
    /// ヘルプを表示。
    ShowHelp,
}

#[derive(Debug)]
pub enum Error {
    TooManyOptions { num: usize },
    UnrecognizedOption { opt: String },
}

impl fmt::Display for Error {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooManyOptions { num } => write!(
                b,
                concat!(
                    "too many / too few options: supplied {} options. ",
                    "this program accepts just one option."
                ),
                num
            ),
            Error::UnrecognizedOption { ref opt } => write!(b, "unknown option {}", opt),
        }
    }
}

impl error::Error for Error {}

/// すべき動作モードを判定する。
/// アルゴリズムは次の通り:
/// 1. コマンドラインオプションは 1 つ以下か？
/// 2. コマンドラインオプションが 1 つだけ存在するか？
///     Some("-w") | Some("--write") => Mode::Write
///     Some("-r") | Some("--read")  => Mode::Read
///     Some("-h") | Some("--help")  => Mode::ShowHelp,
///     Some(_) => mode::Error::UnrecognizedOption,
/// 3. stdin が端末 (isatty) になっているか？
///     Yes => Mode::Read
///     No  => Mode::Write
pub fn detect_mode(mut args: impl Iterator<Item = String>) -> Result<Mode> {
    let arg = args.next();
    let mode = match arg.as_ref().map(|x| &**x) {
        Some("-w") | Some("--write") => Ok(Mode::Write),
        Some("-r") | Some("--read") => Ok(Mode::Read),
        Some("-h") | Some("--help") => Ok(Mode::ShowHelp),
        Some(_) => Err(Error::UnrecognizedOption { opt: arg.unwrap() }),
        None if isatty::stdin_isatty() => Ok(Mode::Read),
        None => Ok(Mode::Write),
    }?;

    if args.next().is_some() {
        return Err(Error::TooManyOptions {
            num: args.count() + 2,
        });
    }

    Ok(mode)
}
