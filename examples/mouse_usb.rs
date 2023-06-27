use hidapi::HidApi;

fn main() {
    let api = HidApi::new().unwrap();
    for device in api.device_list() {
        println!("{:?}", device);
    }
    
    let (vid, pid) = (1133, 49734);
    let device = api.open(vid, pid).unwrap();
    
    let mut buf = [0u8; 8];
    let res = device.read(&mut buf[..]).unwrap();
    println!("Read: {:?}", &buf[..res]);
    
}