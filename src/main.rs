use std::env;
fn convert(args: Vec<String>) -> Vec<String> {
    let mut to_binary = true;
    let mut converted: Vec<String> = Vec::new();
    for (i, arg) in args.iter().enumerate(){
        if i == 0 { continue; }
        if arg == "-r" || arg == "--reverse" {
            to_binary = !to_binary;
            continue;
        }
        if to_binary {
            match arg.parse::<u64>(){
                Err(_) => { continue; },
                Ok(number) => { converted.push(format!("{:b}", number)); continue; },
            }
        }
        match u64::from_str_radix(&arg, 2){
            Err(_) => { continue; },
            Ok(number) => { converted.push(format!("{}", number)); },
        }
    }
    return converted;
}
#[test]
fn test_convert(){
    let test_args = vec![
        String::from("numbin"), String::from("5"), String::from("-r"), String::from("101")
    ];
    let converted = convert(test_args);
    assert_eq!(converted, ["101", "5"]);
}
fn main() {
    let args  = env::args().collect();
    let converted = convert(args);
    for c in converted {
        println!("{c}");
    }
}
