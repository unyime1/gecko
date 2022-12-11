use geckolib::GeckoKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    gecko_mem.exe FILE get KEY
    gecko_mem.exe FILE delete KEY
    gecko_mem.exe FILE insert KEY VALUE
    gecko_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    gecko_mem FILE get KEY
    gecko_mem FILE delete KEY
    gecko_mem FILE insert KEY VALUE
    gecko_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = GeckoKV::open(path).expect("unable to open file");
    store.load().expect("Unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "update" => {
            let value = value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}
