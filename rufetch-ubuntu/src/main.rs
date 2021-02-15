use std::process::Command;
use std::env;


// __--COLORS--__

const RESET: &str = "\x1B[0m";

// ample/coffee theme
const C2: &str = "\x1B[0m\x1B[1m\x1B[38;2;192;103;9m"; // orange
const C1: &str = "\x1B[0m\x1B[1m\x1B[38;2;226;78;78m"; // red
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

fn evar(var: &str) -> String {
    let raw_var = env::var(var);
    
    match raw_var {
        Ok(v) => return v,
        _ => String::new()
    }
}

fn main() {
    let user = cmd!("whoami");
    let host = cmd!("hostname");
    let osname = "Ubuntu";
    let kernel = cmd!("uname", "-sr");
    let uptime = cmd!("uptime", "-p").chars().skip(3).collect::<String>();
    let packages = cmd!("dpkg", "--get-selections").matches("\n").count();
    let shell = cmd!("basename", evar("SHELL"));
    let wm = evar("WM");

    print!(
"           {}_      {}{}{}@{}{}
     ,----{}(_)     OS:{}        {}
   {}_{}/  ---  \\     {}KERNEL:{}    {}
  {}(_) {}|   |  |    {}UPTIME:{}    {}
    {}\\  --- {}_{}/     {}PACKAGES:{}  {}
     {}`----{}(_)     {}SHELL:{}     {}
                  {}WM:{}        {}

",
C1, FONT2, user, FONT1, FONT2, host,
C1, RESET, osname,
C1, C2, FONT1, RESET, kernel,
C1, C2, FONT1, RESET, uptime,
C2, C1, C2, FONT1, RESET, packages,
C2, C1, FONT1, RESET, shell,
FONT1, RESET, wm) 

}


/*

I hate Ubuntu logo
                 _
             ---(_)
         _/  ---  \
        (_) |   |
          \  --- _/
             ---(_)

                 _
           ,----(_)
         _/  ---  \
        (_) |   |  )
          \  --- _/
           `----(_)

                 _
           ,----(_)
         _/  ---  \
        (_) |   | |
          \  --- _/
           `----(_)

    _____
   / ___(_)
 (_)/   \ \
  \ \___/ /
   \____(_)

           __
        __/  \_
    __ / _\__/ \
   /  / /     \ \
   \__| |     | |
      \ \_____/ /
       \___/  \/
           \__/


*/
