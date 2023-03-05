use http_api_client::{
    http::{HeaderMap, Method, StatusCode},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::endpoints::v4::{
    parse_response, render_request, AccessToken, ErrorResponseBody, ParseResponseError,
    RenderRequestError,
};

//
pub struct V4Client<C>
where
    C: Client,
{
    inner: C,
    pub access_token: Option<AccessToken>,
    pub base_url: Option<Url>,
}

impl<C> Clone for V4Client<C>
where
    C: Client + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            access_token: self.access_token.clone(),
            base_url: self.base_url.clone(),
        }
    }
}

impl<C> V4Client<C>
where
    C: Client,
{
    pub fn new(inner: C, access_token: Option<AccessToken>, base_url: Option<Url>) -> Self {
        Self {
            inner,
            access_token,
            base_url,
        }
    }

    pub async fn respond<ReqQuery, ReqBody, RespBody>(
        &self,
        method: Method,
        path: &str,
        query: Option<&ReqQuery>,
        body: Option<&ReqBody>,
    ) -> Result<(StatusCode, RespBody, HeaderMap), V4ClientRespondError<C>>
    where
        ReqQuery: Serialize,
        ReqBody: Serialize,
        RespBody: DeserializeOwned,
    {
        let req = render_request::<ReqQuery, ReqBody>(
            method,
            path,
            query,
            body,
            self.access_token.as_ref(),
            self.base_url.as_ref(),
        )
        .map_err(V4ClientRespondError::RenderRequestError)?;

        let resp = self
            .inner
            .respond(req)
            .await
            .map_err(V4ClientRespondError::RespondError)?;

        let (resp_status, resp_body, resp_headers) =
            parse_response::<RespBody>(resp).map_err(V4ClientRespondError::ParseResponseError)?;

        match resp_body {
            Ok(body) => Ok((resp_status, body, resp_headers)),
            Err(body) => Err(V4ClientRespondError::ResponseStatusCodeNoSuccess(
                resp_status,
                body,
                resp_headers,
            )),
        }
    }
}

//
#[derive(Debug)]
pub enum V4ClientRespondError<C>
where
    C: Client,
{
    RenderRequestError(RenderRequestError),
    RespondError(C::RespondError),
    ParseResponseError(ParseResponseError),
    ResponseStatusCodeNoSuccess(StatusCode, ErrorResponseBody, HeaderMap),
}
impl<C> core::fmt::Display for V4ClientRespondError<C>
where
    C: Client + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl<C> std::error::Error for V4ClientRespondError<C> where C: Client + core::fmt::Debug {}

mod ext {
    use super::*;

    use crate::{
        endpoints::v4::{
            linode_instances::{
                config_create, disk_create, disk_view, ip_addresses_info, linode_boot,
                linode_create, linode_reboot, linode_view, linodes_list,
            },
            linode_types::types_list,
            EmptyMapResponseBody, XListRequestQuery,
        },
        objects::v4::linode_instances::{ConfigId, DiskId, LinodeId},
    };

    impl<C> V4Client<C>
    where
        C: Client,
    {
        //
        //
        //
        pub async fn linode_instances_linodes_list(
            &self,
            page: impl Into<Option<usize>>,
            page_size: impl Into<Option<usize>>,
        ) -> Result<linodes_list::ResponseBody, V4ClientRespondError<C>> {
            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<_, (), _>(
                    Method::GET,
                    "/linode/instances",
                    Some(&XListRequestQuery::new(page, page_size)),
                    None,
                )
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_linode_create<F>(
            &self,
            region: &str,
            r#type: &str,
            mut f: F,
        ) -> Result<linode_create::ResponseBody, V4ClientRespondError<C>>
        where
            F: FnMut(&mut linode_create::RequestBody),
        {
            let mut req_body = linode_create::RequestBody {
                region: region.into(),
                r#type: r#type.into(),
                ..Default::default()
            };
            f(&mut req_body);

            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), _, _>(Method::POST, "/linode/instances", None, Some(&req_body))
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_linode_delete(
            &self,
            linode_id: LinodeId,
        ) -> Result<(), V4ClientRespondError<C>> {
            let (_resp_status, _resp_body, _resp_headers) = self
                .respond::<(), (), EmptyMapResponseBody>(
                    Method::DELETE,
                    format!("/linode/instances/{}", linode_id).as_str(),
                    None,
                    None,
                )
                .await?;
            Ok(())
        }

        pub async fn linode_instances_linode_view(
            &self,
            linode_id: LinodeId,
        ) -> Result<linode_view::ResponseBody, V4ClientRespondError<C>> {
            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), (), _>(
                    Method::GET,
                    format!("/linode/instances/{}", linode_id).as_str(),
                    None,
                    None,
                )
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_linode_boot(
            &self,
            linode_id: LinodeId,
            config_id: ConfigId,
        ) -> Result<(), V4ClientRespondError<C>> {
            let req_body = linode_boot::RequestBody { config_id };
            let (_resp_status, _resp_body, _resp_headers) = self
                .respond::<(), _, EmptyMapResponseBody>(
                    Method::POST,
                    format!("/linode/instances/{}/boot", linode_id).as_str(),
                    None,
                    Some(&req_body),
                )
                .await?;
            Ok(())
        }

        pub async fn linode_instances_linode_reboot(
            &self,
            linode_id: LinodeId,
            config_id: Option<ConfigId>,
        ) -> Result<(), V4ClientRespondError<C>> {
            if let Some(config_id) = config_id {
                let req_body = linode_reboot::RequestBody {
                    config_id: Some(config_id),
                };
                let (_resp_status, _resp_body, _resp_headers) = self
                    .respond::<(), _, EmptyMapResponseBody>(
                        Method::POST,
                        format!("/linode/instances/{}/reboot", linode_id).as_str(),
                        None,
                        Some(&req_body),
                    )
                    .await?;
            } else {
                let (_resp_status, _resp_body, _resp_headers) = self
                    .respond::<(), (), EmptyMapResponseBody>(
                        Method::POST,
                        format!("/linode/instances/{}/reboot", linode_id).as_str(),
                        None,
                        None,
                    )
                    .await?;
            }

            Ok(())
        }

        pub async fn linode_instances_ip_addresses_info(
            &self,
            linode_id: LinodeId,
        ) -> Result<ip_addresses_info::ResponseBody, V4ClientRespondError<C>> {
            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), (), _>(
                    Method::GET,
                    format!("/linode/instances/{}/ips", linode_id).as_str(),
                    None,
                    None,
                )
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_config_create<F>(
            &self,
            linode_id: LinodeId,
            label: &str,
            mut f: F,
        ) -> Result<config_create::ResponseBody, V4ClientRespondError<C>>
        where
            F: FnMut(&mut config_create::RequestBody),
        {
            let mut req_body = config_create::RequestBody {
                label: label.into(),
                ..Default::default()
            };
            f(&mut req_body);

            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), _, _>(
                    Method::POST,
                    format!("/linode/instances/{}/configs", linode_id).as_str(),
                    None,
                    Some(&req_body),
                )
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_disk_create_with_image<F>(
            &self,
            linode_id: LinodeId,
            size: usize,
            image: &str,
            root_pass: &str,
            mut f: F,
        ) -> Result<disk_create::ResponseBody, V4ClientRespondError<C>>
        where
            F: FnMut(&mut disk_create::RequestBodyWithImage),
        {
            let mut req_body = disk_create::RequestBodyWithImage {
                size,
                image: image.into(),
                root_pass: root_pass.into(),
                ..Default::default()
            };
            f(&mut req_body);

            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), _, _>(
                    Method::POST,
                    format!("/linode/instances/{}/disks", linode_id).as_str(),
                    None,
                    Some(&req_body),
                )
                .await?;
            Ok(resp_body)
        }

        pub async fn linode_instances_disk_view(
            &self,
            linode_id: LinodeId,
            disk_id: DiskId,
        ) -> Result<disk_view::ResponseBody, V4ClientRespondError<C>> {
            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), (), _>(
                    Method::GET,
                    format!("/linode/instances/{}/disks/{}", linode_id, disk_id).as_str(),
                    None,
                    None,
                )
                .await?;
            Ok(resp_body)
        }

        //
        //
        //
        pub async fn linode_types_types_list(
            &self,
        ) -> Result<types_list::ResponseBody, V4ClientRespondError<C>> {
            let (_resp_status, resp_body, _resp_headers) = self
                .respond::<(), (), types_list::ResponseBody>(
                    Method::GET,
                    "/linode/types",
                    None,
                    None,
                )
                .await?;
            Ok(resp_body)
        }
    }
}
