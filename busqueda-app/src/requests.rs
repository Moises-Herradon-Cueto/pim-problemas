use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub enum MyRequest {
    Ok { request: Request },
    Error(String),
}

impl MyRequest {
    pub fn post(url: &str) -> Self {
        Self::Ok {
            request: Request::post(url),
        }
    }

    pub fn get(url: &str) -> Self {
        Self::Ok {
            request: Request::get(url),
        }
    }

    pub fn json<T: Serialize + Debug>(self, data: &T) -> Self {
        match self {
            Self::Ok { request } => match request.json(data) {
                Ok(request) => Self::Ok { request },
                Err(err) => {
                    Self::Error(format!("Failed to convert data {data:?} into json\n{err}"))
                }
            },
            Self::Error(_) => self,
        }
    }
    pub fn header(self, key: &str, value: &str) -> Self {
        match self {
            Self::Ok { request } => {
                let new_request = request.header(key, value);
                Self::Ok {
                    request: new_request,
                }
            }
            Self::Error(_) => self,
        }
    }

    pub async fn send_no_parse(self) {
        #![allow(unused_must_use)]
        match self {
            Self::Ok { request } => {
                request.send().await;
            }
            Self::Error(_) => {}
        }
    }

    #[allow(clippy::future_not_send)]
    pub async fn send<R: DeserializeOwned>(self) -> MyResponse<R> {
        match self {
            Self::Ok { request } => match request.send().await {
                Err(err) => MyResponse::Error(format!("Error receiving response: {err}")),
                Ok(response) => MyResponse::<R>::from_gloo_response(response).await,
            },
            Self::Error(err) => MyResponse::Error(err),
        }
    }
}

pub enum MyResponse<T> {
    Ok { response: T },
    Code401,
    Code500(String),
    Error(String),
}

#[allow(clippy::future_not_send)]
impl<T: DeserializeOwned> MyResponse<T> {
    pub async fn from_gloo_response(response: Response) -> Self {
        if response.ok() {
            let response: Result<T, _> = response.json().await;
            // .map_err(|err| )?;
            match response {
                Ok(value) => Self::Ok { response: value },
                Err(err) => Self::Error(format!("Error parsing json\n{err}")),
            }
        } else if response.status() == 500 {
            let body = response.text().await.unwrap_or(String::new());
            Self::Code500(body)
        } else if response.status() == 401 {
            Self::Code401
        } else {
            Self::Error("Error in response".into())
        }
    }

    pub fn into_inner<F: Fn(String) -> T>(self, f: F) -> T {
        use MyResponse::{Code401, Code500, Error, Ok};
        match self {
            Ok { response } => response,
            Code401 => f("No estás autorizado/a a acceder a esto".to_string()),
            Code500(err) => f(err),
            Error(err) => f(err),
        }
    }
}