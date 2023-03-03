pub mod internal {
    use axum::{
        body::Body as HyperBody,
        http::{Request as HttpRequest, Response as HttpResponse},
    };
    use reqwest::{Body as ReqwestBody, Request as ReqwestRequest, Response as ReqwestResponse};

    use crate::context::LinodeApiHttpClient;

    //
    pub async fn reqwest_execute(
        client: &LinodeApiHttpClient,
        request: HttpRequest<HyperBody>,
    ) -> Result<HttpResponse<()>, ()> {
        todo!()
    }
}
