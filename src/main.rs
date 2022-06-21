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
    let pkgs = get_pkgs(dep);

  //  println!("{:?}",pkgs);
  for pkg in &pkgs{
    let lib =build_deps(&pkg);
    
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
let error=true;


'out: while error{
  // println!("{:?}",final_libs);
      let dep = check_extra_dep("",&final_libs.join(":"));

      let pkgs = get_pkgs(&dep);



      for pkg in &pkgs{
        
        let lib =build_deps(&pkg);
        println!("46 {}",lib);
        let new_dep = check_extra_dep("",&(final_libs.join(":")+":"+&lib));
        println!("48 {}",new_dep);
        if new_dep =="None" {
          final_libs.push(lib);
          break 'out;
        }
        if new_dep != dep {
          println!("{}=>{}",dep,lib);
          final_libs.push(lib);
          break
        }
    
      }
    
      
  }

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

fn build_deps(pkg:&str)->String{
  let  p = Command::new("nix-build").arg("--no-link").arg("<nixpkgs>").arg("-A").arg(pkg)
    .output()
    .expect("failed to execute child");
    
    let out_path = std::str::from_utf8(&p.stdout).unwrap().trim();
    out_path.to_owned() + "/lib64:"+&out_path.to_owned() + "/lib"
}

fn get_pkgs(dep:&str)->Vec<String>{


  let  p = Command::new("nix-locate").args(["--db","./db"]).args(["--top-level","--minimal"]).arg(&dep)
    .output()
    .expect("failed to execute child");
   
   let mut pkgs:Vec<String>=  std::str::from_utf8(&p.stdout).unwrap().trim().split("\n").map(|s| s.to_string()).collect();
   
   pkgs.sort_by(|a,b| levenshtein(&dep,&a).cmp(&levenshtein(&dep,&b)));
   pkgs
}


fn check_extra_dep(path:&str,env:&str)->String{
  let  p = Command::new("./firefox/usr/bin/firefox").env("LD_LIBRARY_PATH",env)
  .output()
  .expect("failed to execute child");
  let re = regex::Regex::new(r"\n(.*):\s*cannot open shared object file").unwrap();
    let res = std::str::from_utf8(&p.stderr).unwrap().trim().replace("\t","");
    println!("{}",res);
    let dep = match re.captures(&res){
      Some(x)=>x[1].to_owned(),
      None =>"None".to_owned()
    };
    dep
}