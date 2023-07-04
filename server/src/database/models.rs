use diesel::prelude::*;
use crate::database::{establish_connection, schema};
use crate::database::schema::accounts::dsl;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountEntity {
    pub id: Option<uuid::Uuid>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_initial: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub frozen: Option<bool>
}

pub struct CreateAccountParams {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_initial: Option<String>
}

pub struct FindAccountParams {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_initial: Option<String>
}

pub fn create_account(input: CreateAccountParams) -> QueryResult<AccountEntity> {
    let entity = AccountEntity {
        id: None,
        email: input.email,
        first_name: input.first_name,
        last_name: input.last_name,
        middle_initial: Some(input.middle_initial.unwrap_or(String::from(""))),
        created_at: None,
        updated_at: None,
        frozen: None,
    };

    return diesel::insert_into(dsl::accounts)
        .values(&entity)
        .returning(AccountEntity::as_returning())
        .get_result(&mut establish_connection())
}
