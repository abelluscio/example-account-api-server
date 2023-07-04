// ===================
// Handlers Definition
// Handler functions are defined in this module, and called in the server implementation
// defined in 'main.rs'
// ===================

use chrono::NaiveDateTime;
use diesel::{RunQueryDsl, SelectableHelper};
use tonic::{Status};
use uuid::Uuid;
use crate::{accounts};
use crate::database::{establish_connection, models};
use crate::database::models::{create_account, CreateAccountParams};
use crate::database::schema::accounts::dsl;

fn entity_to_grpc_type<'a>(input: models::AccountEntity) -> accounts::Account {
    return accounts::Account {
        id: input.id.unwrap_or(Uuid::default()).to_string(),
        email: input.email,
        first_name: input.first_name,
        last_name: input.last_name,
        middle_initial: input.middle_initial.unwrap_or(String::from("")).to_string(),
        created_at: input.created_at.unwrap_or(NaiveDateTime::default()).to_string(),
        updated_at: input.updated_at.unwrap_or(NaiveDateTime::default()).to_string(),
        frozen: false,
    };
}

pub fn try_create(request: accounts::CreateRequest) -> Result<accounts::CreateReply, Status> {
    if let Some(input) = request.input {

        // check email is unique...

        // insert into database
        let db_result = create_account(CreateAccountParams {
            email: input.email,
            first_name: input.first_name,
            last_name: input.last_name,
            middle_initial: Some(input.middle_initial)
        });

        if let Ok(result) = db_result {
            // re-format result as a grpc message reply
            return Ok(accounts::CreateReply {
                created: Some(entity_to_grpc_type(result))
            });
        }

        return Err(Status::internal("Failed to create account due to internal error."));
    }

    return Err(Status::invalid_argument("Missing required argument 'input'."));
}

pub fn try_find_one(request: accounts::FindOneRequest) -> Result<accounts::FindOneReply, Status> {
    Ok(accounts::FindOneReply {
        result: Some(accounts::Account {
            id: "123".to_string(),
            email: "email@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Test".to_string(),
            middle_initial: "Q".to_string(),
            created_at: "2023-06-27T12:34:15+0000".to_string(),
            updated_at: "2023-06-27T12:34:15+0000".to_string(),
            frozen: false
        })
    })
}

pub fn try_find_many(request: accounts::FindManyRequest) -> Result<accounts::FindManyReply, Status> {
    Ok(accounts::FindManyReply {
        results: ::prost::alloc::vec::Vec::<accounts::Account>::from(
            vec![
                accounts::Account {
                    id: "123".to_string(),
                    email: "email@test.com".to_string(),
                    first_name: "John".to_string(),
                    last_name: "Test".to_string(),
                    middle_initial: "Q".to_string(),
                    created_at: "2023-06-27T12:34:15+0000".to_string(),
                    updated_at: "2023-06-27T12:34:15+0000".to_string(),
                    frozen: false
                }
            ]
        )
    })
}

pub fn try_update_one(request: accounts::UpdateOneRequest) -> Result<accounts::UpdateOneReply, Status> {
    Ok(accounts::UpdateOneReply {
        id: request.id,
        before: Some(accounts::Account {
            id: "123".to_string(),
            email: "email@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Test".to_string(),
            middle_initial: "Q".to_string(),
            created_at: "2023-06-27T12:34:15+0000".to_string(),
            updated_at: "2023-06-27T12:34:15+0000".to_string(),
            frozen: false
        }),
        after: Some(accounts::Account {
            id: "123".to_string(),
            email: "email@test.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Test".to_string(),
            middle_initial: "Q".to_string(),
            created_at: "2023-06-27T12:34:15+0000".to_string(),
            updated_at: "2023-06-27T12:34:15+0000".to_string(),
            frozen: false
        })
    })
}

pub fn try_delete_one(request: accounts::DeleteOneRequest) -> Result<accounts::DeleteOneReply, Status> {
    Ok(accounts::DeleteOneReply {
        id: request.id,
        success: true
    })
}

pub fn try_delete_many(request: accounts::DeleteManyRequest) -> Result<accounts::DeleteManyReply, Status> {
    Ok(accounts::DeleteManyReply {
        results: ::prost::alloc::vec::Vec::<accounts::DeleteOneReply>::from(
            vec![] // TODO: map request IDs here?
        )
    })
}
