use std::process::Command;
use std::env;

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
    let host = cmd!("cat", "/etc/hostname");
    let osname = "Void";
    let kernel = cmd!("uname", "-sr");
    let uptime = cmd!("uptime", "-p").chars().skip(3).collect::<String>();
    let packages = cmd!("xbps-query", "-l").matches("\n").count();
    let shell = cmd!("basename", env::var("SHELL").unwrap());
    let wm = env::var("WM").unwrap();

    // __--COLORS--__

    let reset = "\x1B[0m";

    // ample/coffee theme
    let c1 = "\x1B[0m\x1B[1m\x1B[38;2;192;103;9m"; // orange
    let c2 = "\x1B[0m\x1B[1m\x1B[38;2;226;78;78m"; // red
    let font1 = "\x1B[0m\x1B[1m\x1B[38;2;226;78;78m";
    let font2 = "\x1B[0m\x1B[1m\x1B[38;2;192;103;9m";
    // ample/coffee theme

    // --__COLORS__--

    print!(
"      {}_______      {}{}{}@{}{}
   {}_ \\______ -     {}OS:{}        {}
  {}| \\  {}___{}  \\ |    {}KERNEL:{}    {}
  {}| | {}/   \\{} | |    {}UPTIME:{}    {}
  {}| | {}\\___/{} | |    {}PACKAGES:{}  {}
  {}| \\______ \\_|    {}SHELL:{}     {}
   {}-_______\\       {}WM:{}        {}{}

",
        c1, font2, user, font1, font2, host,
        c1, font1, reset, osname,
        c1, c2, c1, font1, reset, kernel,
        c1, c2, c1, font1, reset, uptime,
        c1, c2, c1, font1, reset, packages,
        c1, font1, reset, shell,
        c1, font1, reset, wm, reset)




}


