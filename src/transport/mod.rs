mod production;
#[cfg(test)]
mod testing;

use Error;
use hyper;
use serde;

pub use self::production::Transport;
#[cfg(test)]
pub use self::testing::{StubRequest, StubRequestMaker, StubResponse};

pub trait Action {
    type Output;
    type State;

    fn create_request<R>(self, request_maker: R) -> Result<(R::Request, Self::State), Error>
        where R: RequestMaker;

    fn handle_response<R>(response: R, state: Self::State) -> Result<Self::Output, Error>
        where R: Response;
}

pub trait RequestMaker {
    type Request: Request;
    fn make_request<P>(&self,
                       method: hyper::method::Method,
                       url_path_components: P)
                       -> Self::Request
        where P: Iterator<Item = String>;
}

pub trait Request {
    fn set_body(self, body: Vec<u8>) -> Self;
    fn set_content_type_json(self) -> Self;
}

pub trait Response {
    fn status_code(&self) -> hyper::status::StatusCode;
    fn json_decode_content<T: serde::Deserialize>(self) -> Result<T, Error>;
}
