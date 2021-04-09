use actix_http::Response;
use actix_web::{HttpResponse, Responder, HttpRequest};
use rbatis::core::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use futures::future::{ok, Ready};

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


/// Returns Resp<T> json.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use newbbs::base::resp::resp;
/// use newbbs::base::resp::RespErr::SimpleError;
/// use newbbs::base::resp::RespErr::CodeError;
///
/// #[derive(Serialize, Deserialize, Clone, Debug)]
/// pub struct UserDTO {
///     pub username: String,
///     pub password: String,
/// }
///
/// let data = UserDTO {
///     username: "xiaoming".into(),
///     password: "123456".into(),
/// };
///
/// resp(&Ok(data));
/// resp(&Err(SimpleError("服务器内部异常".into())));
/// resp(&Err(CodeError("1001".into(), "账号未登录".into())));
///
/// ```
pub fn resp<T>(arg: &Result<T>) -> Response where T: Serialize + DeserializeOwned + Clone {
    let res = match arg {
        Ok(data) => Resp { code: "0".into(), msg: None, data: Some(data) },
        Err(e) => {
            match e {
                RespErr::SimpleError(e) => Resp { code: "1111".into(), msg: Some(e.clone()), data: None },
                RespErr::CodeError(code, e) => Resp { code: code.clone(), msg: Some(e.clone()), data: None },
            }
        }
    };

    HttpResponse::Ok().json2(&res)
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
        Self {
            inner: resp(&data)
        }
    }
}

impl From<RespErr> for JsonResponse{
    fn from(data: RespErr) -> Self {
        Self {
            inner: resp::<()>(&Err(data))
        }
    }
}


#[test]
fn test() {
    let res: JsonResponse = SimpleError("error".into()).into();
    println!("{:?}", res.inner);
    let res: JsonResponse = CodeError("123".into(), "error".into()).into();
    println!("{:?}", res.inner);
}