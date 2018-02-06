// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait UserService {
    fn get_user(&self, o: ::grpc::RequestOptions, p: super::messages::UserRequest) -> ::grpc::SingleResponse<super::messages::User>;

    fn get_users(&self, o: ::grpc::RequestOptions, p: super::messages::Empty) -> ::grpc::SingleResponse<super::messages::UsersResponse>;
}

// client

pub struct UserServiceClient {
    grpc_client: ::grpc::Client,
    method_GetUser: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messages::UserRequest, super::messages::User>>,
    method_GetUsers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::messages::Empty, super::messages::UsersResponse>>,
}

impl UserServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        UserServiceClient {
            grpc_client: grpc_client,
            method_GetUser: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/UserService/GetUser".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetUsers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/UserService/GetUsers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            UserServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            UserServiceClient::with_client(c)
        })
    }
}

impl UserService for UserServiceClient {
    fn get_user(&self, o: ::grpc::RequestOptions, p: super::messages::UserRequest) -> ::grpc::SingleResponse<super::messages::User> {
        self.grpc_client.call_unary(o, p, self.method_GetUser.clone())
    }

    fn get_users(&self, o: ::grpc::RequestOptions, p: super::messages::Empty) -> ::grpc::SingleResponse<super::messages::UsersResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetUsers.clone())
    }
}

// server

pub struct UserServiceServer;


impl UserServiceServer {
    pub fn new_service_def<H : UserService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/UserService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/UserService/GetUser".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_user(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/UserService/GetUsers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_users(o, p))
                    },
                ),
            ],
        )
    }
}
