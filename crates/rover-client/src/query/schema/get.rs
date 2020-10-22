use crate::blocking::Client;
use crate::RoverClientError;
use graphql_client::*;

// I'm not sure where this should live long-term
/// this is because of the custom GraphQLDocument scalar in the schema
type GraphQLDocument = String;

#[derive(GraphQLQuery)]
// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[graphql(
    query_path = "src/query/schema/get.graphql",
    schema_path = "schema.graphql",
    response_derives = "PartialEq, Debug, Serialize, Deserialize",
    deprecated = "warn"
)]
/// This struct is used to generate the module containing `Variables` and
/// `ResponseData` structs.
/// Snake case of this name is the mod name. i.e. get_schema_query
pub struct GetSchemaQuery;

/// The main function to be used from this module. This function fetches a
/// schema from apollo studio and returns it in either sdl (default) or json format
pub fn run(
    variables: get_schema_query::Variables,
    client: Client,
) -> Result<String, RoverClientError> {
    let response_data = execute_query(client, variables)?;

    // get the schema document from ResponseData
    // It's under response_data.Option(service).Option(schema).document
    let schema = match response_data.service {
        Some(service_data) => match service_data.schema {
            Some(sch) => sch,
            None => {
                return Err(RoverClientError::ResponseError {
                    msg: "No schema found for this variant".to_string(),
                })
            }
        },
        None => {
            return Err(RoverClientError::ResponseError {
                msg: "No service found".to_string(),
            })
        }
    }
    .document;

    // if we want json, we can parse & serialize it here

    Ok(schema)
}

fn execute_query(client: Client, variables: get_schema_query::Variables) -> Result<get_schema_query::ResponseData, RoverClientError> {
    let res = client.post::<GetSchemaQuery>(variables)?;
    if let Some(data) = res {
        Ok(data)
    } else {
        Err(RoverClientError::ResponseError {
            msg: "Error fetching schema. No data in response".to_string(),
        })
    }
}

fn get_schema_from_response_data(response_data: get_schema_query::ResponseData) -> Result<String, RoverClientError> {
    // match response_data.service {
    //     Some(service) => { service },
    //     None => {
    //         Err(RoverClientError::ResponseError {
    //             msg: "No service found".to_string(),
    //         })
    //     }
    // };

    // if let Some(schema) = service_data.schema {
    //     schema
    // } else {
    //     Err(RoverClientError::ResponseError {
    //         msg: "No schema found for this variant".to_string(),
    //     })
    // }

    
    let schema = match response_data.service {
        Some(service_data) => match service_data.schema {
            Some(sch) => sch,
            None => {
                return Err(RoverClientError::ResponseError {
                    msg: "No schema found for this variant".to_string(),
                })
            }
        },
        None => {
            return Err(RoverClientError::ResponseError {
                msg: "No service found".to_string(),
            })
        }
    };

    Ok(schema.document)
    // .document
}
