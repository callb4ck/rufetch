use std::process::Command;
use std::env;


// __--COLORS--__

const RESET: &str = "\x1B[0m";

// ample/coffee theme
const C1: &str = "\x1B[0m\x1B[1m\x1B[38;2;192;103;9m"; // orange
const C2: &str = "\x1B[0m\x1B[1m\x1B[38;2;226;78;78m"; // red
const FONT1: &str = "\x1B[0m\x1B[1m\x1B[38;2;226;78;78m";
const FONT2: &str = "\x1B[0m\x1B[1m\x1B[38;2;192;103;9m";
// ample/coffee theme

// --__COLORS__--

macro_rules! cmd {
    ($cmd:expr) => {
        String::from_utf8(
            Command::new($cmd)
            .output()
            .unwrap()
            .stdout
        ).unwrap().trim().to_string()
    };

    ($cmd:expr, $($arg:expr),*) => {
        String::from_utf8(
            Command::new($cmd)
            $(
                .arg($arg)
            )*
            .output()
            .unwrap()
            .stdout
        ).unwrap().trim().to_string()
    }
}

fn main() {
    let user = cmd!("whoami");
    let host = cmd!("hostname");
    let osname = "Void";
    let kernel = cmd!("uname", "-sr");
    let uptime = cmd!("uptime", "-p").chars().skip(3).collect::<String>();
    let packages = cmd!("xbps-query", "-l").matches("\n").count();
    let shell = cmd!(
        "basename",
        env::var("SHELL").unwrap_or_else(|_| String::new())
    );
    let wm = env::var("WM").unwrap_or_else(|_| String::new());

    #[rustfmt::skip]
    {
        print!(
"      {}_______      {}{}{}@{}{}
   {}_ \\______ -     {}OS:{}        {}
  {}| \\  {}___{}  \\ |    {}KERNEL:{}    {}
  {}| | {}/   \\{} | |    {}UPTIME:{}    {}
  {}| | {}\\___/{} | |    {}PACKAGES:{}  {}
  {}| \\______ \\_|    {}SHELL:{}     {}
   {}-_______\\       {}WM:{}        {}{}

",
C1, FONT2, user, FONT1, FONT2, host,
C1, FONT1, RESET, osname,
C1, C2, C1, FONT1, RESET, kernel,
C1, C2, C1, FONT1, RESET, uptime,
C1, C2, C1, FONT1, RESET, packages,
C1, FONT1, RESET, shell,
C1, FONT1, RESET, wm, RESET)
    };

}
