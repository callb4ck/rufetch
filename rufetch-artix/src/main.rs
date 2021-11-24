use std::env;
use std::process::Command;

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

    let host = std::fs::read_to_string("/etc/hostname")
        .unwrap_or(String::from(""))
        .strip_suffix('\n')
        .unwrap_or("")
        .to_string();

    let osname = "Artix";
    let kernel = cmd!("uname", "-sr");
    let uptime = cmd!("uptime", "-p").chars().skip(3).collect::<String>();
    let packages = cmd!("pacman", "-Q").matches("\n").count()+1;
    let shell = cmd!(
        "basename",
        env::var("SHELL").unwrap_or_else(|_| String::new())
    );
    let wm = env::var("WM").unwrap_or_else(|_| String::new());

    println!(
        r"{C1}      /\         {FONT2}{USER}{FONT1}@{FONT2}{HOST}
{C1}     /  \        {FONT1}OS:        {RESET}{OS}
{C1}    /`'.,\       {FONT1}KERNEL:    {RESET}{KERNEL}
{C1}   /  {C2}   ',      {FONT1}UPTIME:    {RESET}{UPTIME}
{C2}  /      ,`\     {FONT1}PACKAGES:  {RESET}{PACKAGES}
{C2} /   ,.'`.  \    {FONT1}SHELL:     {RESET}{SHELL}
{C2}/.,'`     `'.\   {FONT1}WM:        {RESET}{WM}
{RESET}",
        C1 = C1,
        C2 = C2,
        FONT1 = FONT1,
        FONT2 = FONT2,
        RESET = RESET,
        USER = user,
        HOST = host,
        OS = osname,
        KERNEL = kernel,
        UPTIME = uptime,
        PACKAGES = packages,
        SHELL = shell,
        WM = wm
    );
}
