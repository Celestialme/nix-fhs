use std::process::{Command, Stdio,ChildStdout,ChildStdin};
extern crate strsim;
use strsim::levenshtein;
fn main() {
    let mut  final_libs=Vec::new();
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1].to_owned();
    let deps = find_unresolved_deps(&path,"");
 println!("{:?}",deps);
for dep in &deps{
    let  p = Command::new("nix-locate").args(["--db","./db"]).args(["--top-level","--minimal"]).arg(&dep)
    .output()
    .expect("failed to execute child");
   
   let mut pkgs:Vec<String>=  std::str::from_utf8(&p.stdout).unwrap().trim().split("\n").map(|s| s.to_string()).collect();
   
   pkgs.sort_by(|a,b| levenshtein(&dep,&a).cmp(&levenshtein(&dep,&b)));

  //  println!("{:?}",pkgs);
  for pkg in &pkgs{
    
    let  p = Command::new("nix-build").arg("--no-link").arg("<nixpkgs>").arg("-A").arg(&pkg)
    .output()
    .expect("failed to execute child");
    
    let out_path = std::str::from_utf8(&p.stdout).unwrap().trim();
    let lib = out_path.to_owned() + "/lib";
    let new_deps = find_unresolved_deps(path,&lib);
  
    if new_deps.len() < deps.len() {
      println!("{}=>{}",dep,lib);
      final_libs.push(lib);
      break
    }

  }

   
 
}
 final_libs.dedup();
println!("LD_LIBRARY_PATH={}",final_libs.join(":"));
}


fn find_unresolved_deps(path:&str,env:&str)->Vec<String>{
  let re = regex::Regex::new(r"(.*)=>\s*not found").unwrap();
  let  p = Command::new("ldd").arg(&path).env("LD_LIBRARY_PATH", env)
  .output()
  .expect("failed to execute child");

    let res =  std::str::from_utf8(&p.stdout).unwrap().trim().replace("\t","");
    let mut deps = Vec::new();
 
    for caps in re.captures_iter(&res) {
       
       deps.push(caps[1].to_string().trim().to_owned());
       }
      deps
}