use mecha_trustzone_ctrl::TrustZoneCtrl;


fn main(){
    //use trutstm to read cert 
    let trustm = TrustZoneCtrl::new();
    let output_cert = "hello_world.crt";
    let region = "0xe0e0";
    let cert = trustm.read_trustzone_cert(output_cert, region);
    println!("cert: {:?}", cert);
}