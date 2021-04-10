
use actix_http::Response;
use actix_web::{HttpResponse, Responder, HttpRequest};
use rbatis::core::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use futures::future::{ok, Ready};
use crate::base::resp::RespErr::{SimpleError, CodeError};

/// response struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resp<T> {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RespErr {
    SimpleError(String),
    CodeError(String, String),
}

impl From<Error> for RespErr {
    fn from(e: Error) -> Self {
        RespErr::SimpleError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RespErr>;


/// Returns JsonResponse.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use newbbs::base::resp::{JsonResponse};
/// use newbbs::base::resp::RespErr::SimpleError;
///
/// #[derive(Serialize, Deserialize, Clone, Debug)]
/// pub struct UserDTO {
///     pub username: String,
///     pub password: String,
/// }
///
/// pub async fn err() -> JsonResponse {
///     SimpleError(String::from("some error")).into()
/// }
///
/// pub async fn login(user: UserDTO) -> JsonResponse {
///     Ok(user).into()
/// }
///
/// ```

pub fn resp_json<T>(arg: &Result<T>) -> JsonResponse where T: Serialize + DeserializeOwned + Clone {
    let res = match arg {
        Ok(data) => Resp { code: "0".into(), msg: None, data: Some(data) },
        Err(e) => {
            match e {
                RespErr::SimpleError(e) => Resp { code: "111111".into(), msg: Some(e.clone()), data: None },
                RespErr::CodeError(code, e) => Resp { code: code.clone(), msg: Some(e.clone()), data: None },
            }
        }
    };

    JsonResponse {
        inner: HttpResponse::Ok().json2(&res)
    }
}

pub struct JsonResponse {
    inner: Response,
}

impl Responder for JsonResponse {
    type Error = ();
    type Future = Ready<std::result::Result<Response, ()>>;

    #[inline]
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(self.inner)
    }
}

impl<T> From<Result<T>> for JsonResponse where T: Serialize + DeserializeOwned + Clone {
    fn from(data: Result<T>) -> Self {
        resp_json(&data)
    }
}

impl From<RespErr> for JsonResponse {
    fn from(data: RespErr) -> Self {
        resp_json::<()>(&Err(data))
    }
}

pub fn code_error<T: Into<String>>(code: T, err: T) -> JsonResponse {
    CodeError(code.into(), err.into()).into()
}

pub fn simple_error<T: Into<String>>(err: T) -> JsonResponse {
    SimpleError(err.into()).into()
}


#[cfg(test)]
mod test {
    use crate::base::resp::JsonResponse;
    use crate::base::resp::RespErr::{SimpleError, CodeError};

    #[test]
    fn test() {
        let res: JsonResponse = SimpleError("error".into()).into();
        println!("{:?}", res.inner);
        let res: JsonResponse = CodeError("123".into(), "error".into()).into();
        println!("{:?}", res.inner);
    }
}