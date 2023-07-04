mod database;
mod handlers;

use tonic::{Request, Response, Status, transport::Server};
use tonic_reflection;
use accounts::{
    account_api_server::{AccountApi, AccountApiServer}
};

pub mod accounts {
    tonic::include_proto!("accounts"); // specify package name

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("../accounts_descriptor.bin");
}

#[derive(Debug, Default)]
pub struct AccountService {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(accounts::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let address = "[::1]:8080".parse().unwrap();
    let account_service = AccountService::default();

    Server::builder()
        .add_service(reflection_service)
        .add_service(AccountApiServer::new(account_service))
        .serve(address)
        .await?;
    Ok(())

}

fn handle_result<T>(result: Result<T, Status>) -> Result<Response<T>, Status> {
    if result.is_err() {
        return Err(Status::from(result.err().unwrap()));
    }

    Ok(Response::new(result.unwrap()))
}

#[tonic::async_trait]
impl AccountApi for AccountService {
    async fn create(&self, request: Request<accounts::CreateRequest>) -> Result<Response<accounts::CreateReply>, Status> {
        return handle_result::<accounts::CreateReply>(handlers::try_create(request.into_inner()));
    }

    async fn find_one(&self, request: Request<accounts::FindOneRequest>) -> Result<Response<accounts::FindOneReply>, Status> {
        return handle_result::<accounts::FindOneReply>(handlers::try_find_one(request.into_inner()));
    }

    async fn find_many(&self, request: Request<accounts::FindManyRequest>) -> Result<Response<accounts::FindManyReply>, Status> {
        return handle_result::<accounts::FindManyReply>(handlers::try_find_many(request.into_inner()));
    }

    async fn update_one(&self, request: Request<accounts::UpdateOneRequest>) -> Result<Response<accounts::UpdateOneReply>, Status> {
        return handle_result::<accounts::UpdateOneReply>(handlers::try_update_one(request.into_inner()));
    }

    async fn delete_one(&self, request: Request<accounts::DeleteOneRequest>) -> Result<Response<accounts::DeleteOneReply>, Status> {
        return handle_result::<accounts::DeleteOneReply>(handlers::try_delete_one(request.into_inner()));
    }

    async fn delete_many(&self, request: Request<accounts::DeleteManyRequest>) -> Result<Response<accounts::DeleteManyReply>, Status> {
        return handle_result::<accounts::DeleteManyReply>(handlers::try_delete_many(request.into_inner()));
    }
}
