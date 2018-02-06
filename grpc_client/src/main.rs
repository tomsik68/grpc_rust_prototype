extern crate grpc_common;
extern crate grpc;

use grpc_common::services_grpc::*;
use grpc_common::messages::*;

fn main() {
    let client = UserServiceClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let resp = client.get_users(grpc::RequestOptions::new(), Empty::new()).wait();
    let (m1, users_resp, m2) = resp.expect("request failed");
    println!("Metadata 1: {:?}", m1);
    println!("Users Response: {:?}", users_resp);
    println!("Metadata 2: {:?}", m2);

}
