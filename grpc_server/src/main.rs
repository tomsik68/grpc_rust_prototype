extern crate grpc;
extern crate protobuf;
extern crate grpc_common;
extern crate bytes;

use bytes::Bytes;
use std::thread;
use grpc::Metadata;
use grpc::MetadataKey;
use std::time::Instant;
use protobuf::RepeatedField;
use grpc_common::services_grpc::*;
use grpc_common::messages::*;

struct UserServiceImpl;

impl UserService for UserServiceImpl {
    fn get_user(&self, _o: ::grpc::RequestOptions, _p: UserRequest) -> ::grpc::SingleResponse<User> {
        ::grpc::SingleResponse::completed(User::new())
    }

    fn get_users(&self, _o: ::grpc::RequestOptions, _p: Empty) -> ::grpc::SingleResponse<UsersResponse> {
        let mut meta = Metadata::new();
        let time = Instant::now().elapsed().as_secs().to_string();
        meta.add(MetadataKey::from("recv_time"), Bytes::from(time));
        let mut resp = UsersResponse::new();
        let mut users = Vec::new();
        for i in 0..10 {
            let mut u = User::new();
            u.set_user_id(i.to_string());
            u.set_username(String::from("User User"));
            users.push(u);
        }
        resp.set_users(RepeatedField::from_vec(users));
        ::grpc::SingleResponse::completed_with_metadata(meta, resp)
    }
}

fn main() {
    let mut builder = grpc::ServerBuilder::new_plain();
    builder.http.set_port(50051);
    builder.add_service(UserServiceServer::new_service_def(UserServiceImpl));
    builder.http.set_cpu_pool_threads(4);
    let _server = builder.build().expect("Error starting server");
    loop {
        thread::park();
    }
}
