use std::process::{Command, Stdio,ChildStdout,ChildStdin};
fn main() {
    let re = regex::Regex::new(r"(.*)=>\s*not found").unwrap();
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1].to_owned();
    let mut deps = Vec::new();
    let  p = Command::new("ldd").arg(&path)
  .output()
  .expect("failed to execute child");

    let res =  std::str::from_utf8(&p.stdout).unwrap().trim().replace("\t","");
    for caps in re.captures_iter(&res) {
       
       deps.push(caps[1].to_string().trim().to_owned());
       }
 println!("{:?}",deps);
for dep in deps{
    let  p = Command::new("nix-locate").args(["--db","./db"]).arg("--top-level").arg(&dep)
    .output()
    .expect("failed to execute child");
   
   let res:Vec<String>=  std::str::from_utf8(&p.stdout).unwrap().trim().split("\n").map(|s| s.to_string()).collect();
   println!("{:?}",res[0]);
}

}
