// use graphql_client::{GraphQLQuery, Response};
// use std::error::Error;
// use reqwest;
//
// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "tests/unions/union_schema.graphql",
//     query_path = "tests/unions/union_query.graphql",
//     response_derives = "Debug",
// )]
// pub struct UnionQuery;
//
// async fn perform_my_query(variables: union::Variables) -> Result<(), Box<dyn Error>> {
//
//     // this is the important line
//     let request_body = UnionQuery::build_query(variables);
//
//     let client = reqwest::Client::new();
//     let mut res = client.post("/graphql").json(&request_body).send().await?;
//     let response_body: Response<union_query::ResponseData> = res.json().await?;
//     println!("{:#?}", response_body);
//     Ok(())
// }