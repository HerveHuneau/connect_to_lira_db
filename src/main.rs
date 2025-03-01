use std::process::Command;

fn main() {
    Command::new("pgcli")
        .arg("postgres://postgres:postgres@postgres:15432/lira")
        .status()
        .expect("pgcli didn't finish successfully");
}
