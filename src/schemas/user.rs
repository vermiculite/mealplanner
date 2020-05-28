use crate::schemas::product::Product;
use crate::schemas::root::Context;
use bson::doc;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

#[juniper::object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    async fn products(&self, context: &Context) -> Vec<Product> {
        let cursor = context.users.find(doc!{"user_id": self.id}, None).await?;
        let docs: Vec<Product> = cursor.map(|doc| doc.unwrap()).collect();
        docs
    }
}