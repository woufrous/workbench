mod usbtmc;

use std::path::Path;

fn main() -> usbtmc::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("USAGE: {} dev cmd", args[0]);
        return usbtmc::Result::Ok(());
    }

    let dev = usbtmc::UsbTmcDevice::open(Path::new(&args[1]))?;
    let version = dev.get_api_version()?;
    println!("USB-TMC API version: {}", version);

    let nwritten = dev.write(args[2].as_bytes())?;

    let mut buf = [0; 256];
    let nread = dev.read(&mut buf)?;

    println!("{} bytes written; {} bytes read", nwritten, nread);
    println!("{}", std::str::from_utf8(&buf).unwrap());

    usbtmc::Result::Ok(())
}
