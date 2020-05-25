use juniper::{object, Context as JuniperContext, FieldResult, RootNode};

#[derive(Clone, Debug)]
pub struct Fruit {
    id: String,
    name: String,
    country: String,
}

impl Fruit {
    pub fn new(id: String, name: String, country: String) -> Fruit {
        Fruit { id, name, country }
    }
}

#[object]
#[graphql(description = "A Project returns struct")]
impl Fruit {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn country(&self) -> String {
        self.country.clone()
    }

    pub fn reply(&self) -> String {
        format!("{} from {}", self.name, self.country)
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub fruits: Vec<Fruit>,
}

impl JuniperContext for Context {}

pub struct Query;

#[object(Context = Context)]
impl Query {
    fn all_fruits(&self, context: &Context) -> FieldResult<Vec<Fruit>> {
        Ok(context.fruits.clone())
    }
}

pub struct Mutation;

#[object(Context = Context)]
impl Mutation {}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
