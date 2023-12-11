use std::fs;
use std::fs::File;
use std::io::Write;

pub fn convert_APX_to_ICCMA2023(mut filepath : String) {
    let content  = fs::read_to_string(filepath.clone()).unwrap();
    
    filepath.push_str(".af");
    let mut f = File::create(filepath).unwrap();
    let mut nb_arg = 0;
    let mut begin = true;
    for line in content.lines() {
        if line.starts_with("arg") {
            nb_arg+=1;
        }
        else if line.starts_with("att") {
            if begin {
                begin = false;
                let mut buf = String::from("p af ");
                buf.push_str(nb_arg.to_string().as_str());
                f.write(buf.as_bytes());
                f.write(&[b'\n']);
            }
            let mut buff = line.strip_prefix("att(a").unwrap();
            let buff2 = buff.strip_suffix(").").unwrap();
            let buff2 = buff2.replace(",a", " ");
            println!("{}", buff2);
            f.write(buff2.as_bytes()).unwrap();
            f.write(&[b'\n']);
        }
        else if line.starts_with('#') {
            f.write(line.as_bytes()).unwrap();
        }
    }
}
pub fn convert_ICCMA2023_to_APX(mut filepath : String) {
    let content  = fs::read_to_string(filepath.clone()).expect("BIZARRE");
    filepath.push_str(".apx");
    let mut f = File::create(filepath).unwrap();
    let mut iter = content.lines();
    let nb_arg = iter.next().unwrap().split_ascii_whitespace().nth(2).unwrap().parse().unwrap();
    println!("{}", nb_arg);
    for i in 1..nb_arg {
        let mut buff = String::from("arg(a");
        buff.push_str(i.to_string().as_str());
        buff.push_str(").");
        f.write(buff.as_bytes()).unwrap();
        let _ = f.write(&[b'\n']);
    }
    for line in iter {
        let split : Vec<&str> = line.split_ascii_whitespace().collect();
        let n1 = split[0];
        let n2 = split[1];
        let mut buff = String::from("att(a");
        buff.push_str(n1.to_string().as_str());
        buff.push_str(",a");
        buff.push_str(n2.to_string().as_str());
        buff.push_str(").");
        f.write(buff.as_bytes()).unwrap();
        let _ = f.write(&[b'\n']);
    }
}