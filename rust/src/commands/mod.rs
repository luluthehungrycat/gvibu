pub mod basename;
pub mod cat;
pub mod dirname;
pub mod head;
pub mod wc;
pub mod echo_cmd;
pub mod false_cmd;
pub mod pwd_cmd;
pub mod true_cmd;
pub mod yes;
pub mod printenv;
pub mod sleep;
pub mod touch;
pub mod seq;
pub mod which_cmd;
pub mod uname;
pub mod env_cmd;
pub mod whoami;
pub mod link;
pub mod unlink;
pub mod tee;
pub mod mkdir;
pub mod rmdir;

pub mod hostname;
pub mod logname;
pub mod readlink;
pub mod realpath;
pub mod uniq;
pub mod uptime;
pub mod id;
pub mod who;
pub mod kill;
pub mod cut;
pub mod tr;
pub mod mv;
pub mod rm;
pub mod ln;
pub mod chmod;
pub mod chown;
pub mod sort;
pub mod test_cmd;

pub mod grep;
pub mod ls;
pub mod tail;
pub mod tac;
pub mod fold;
pub mod comm;
pub mod join;
pub mod nl;
pub mod shuf;
pub mod sum;

pub mod cp;
pub mod printf;
pub mod date;
pub mod expr;
pub mod split;
pub mod du;
pub mod df;

pub const BINARY_NAMES: &[&str] = &["gvibu", "gvibu-ref"];

pub struct Command {
    pub names: &'static [&'static str],
    pub run: fn(stdout: &mut dyn std::io::Write, args: &[String]) -> i32,
}

pub const COMMANDS: &[Command] = &[
    Command { names: &["true"], run: true_cmd::run },
    Command { names: &["false"], run: false_cmd::run },
    Command { names: &["echo"], run: echo_cmd::run },
    Command { names: &["pwd"], run: pwd_cmd::run },
    Command { names: &["basename"], run: basename::run },
    Command { names: &["dirname"], run: dirname::run },
    Command { names: &["cat"], run: cat::run },
    Command { names: &["wc"], run: wc::run },
    Command { names: &["head"], run: head::run },
    Command { names: &["yes"], run: yes::run },
    Command { names: &["printenv"], run: printenv::run },
    Command { names: &["sleep"], run: sleep::run },
    Command { names: &["touch"], run: touch::run },
    Command { names: &["seq"], run: seq::run },
    Command { names: &["which"], run: which_cmd::run },
    Command { names: &["uname"], run: uname::run },
    Command { names: &["env"], run: env_cmd::run },
    Command { names: &["whoami"], run: whoami::run },
    Command { names: &["link"], run: link::run },
    Command { names: &["unlink"], run: unlink::run },
    Command { names: &["tee"], run: tee::run },
    Command { names: &["mkdir"], run: mkdir::run },
    Command { names: &["rmdir"], run: rmdir::run },
    Command { names: &["hostname"], run: hostname::run },
    Command { names: &["logname"], run: logname::run },
    Command { names: &["readlink"], run: readlink::run },
    Command { names: &["realpath"], run: realpath::run },
    Command { names: &["uniq"], run: uniq::run },
    Command { names: &["uptime"], run: uptime::run },
    Command { names: &["id"], run: id::run },
    Command { names: &["who"], run: who::run },
    Command { names: &["kill"], run: kill::run },
    Command { names: &["cut"], run: cut::run },
    Command { names: &["tr"], run: tr::run },
    Command { names: &["mv"], run: mv::run },
    Command { names: &["rm"], run: rm::run },
    Command { names: &["ln"], run: ln::run },
    Command { names: &["chmod"], run: chmod::run },
    Command { names: &["chown"], run: chown::run },
    Command { names: &["sort"], run: sort::run },
    Command { names: &["test", "["], run: test_cmd::run },
    Command { names: &["grep"], run: grep::run },
    Command { names: &["ls"], run: ls::run },
    Command { names: &["tail"], run: tail::run },
    Command { names: &["tac"], run: tac::run },
    Command { names: &["fold"], run: fold::run },
    Command { names: &["comm"], run: comm::run },
    Command { names: &["join"], run: join::run },
    Command { names: &["nl"], run: nl::run },
    Command { names: &["shuf"], run: shuf::run },
    Command { names: &["sum"], run: sum::run },
    Command { names: &["cp"], run: cp::run },
    Command { names: &["printf"], run: printf::run },
    Command { names: &["date"], run: date::run },
    Command { names: &["expr"], run: expr::run },
    Command { names: &["split"], run: split::run },
    Command { names: &["du"], run: du::run },
    Command { names: &["df"], run: df::run },
];

pub fn lookup(name: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|cmd| cmd.names.contains(&name))
}
