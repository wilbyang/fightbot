// Copyright 2025 Cloudflare, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use pingora::{Error, ErrorType};
use url::Url;

use async_trait::async_trait;
use bytes::Bytes;


use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};

use crate::{html_processor::HtmlProcessor, id_mapping::IdMapping};
use crate::config::{Config, Route};

fn check_login(req: &pingora_http::RequestHeader) -> bool {
    // implement you logic check logic here
    req.headers.get("Authorization").map(|v| v.as_bytes()) == Some(b"password")
}

pub struct IdObfuscationProxy {
    config: Arc<Config>,
    id_mapping: Arc<IdMapping>,
}

impl IdObfuscationProxy {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            id_mapping: Arc::new(IdMapping::new()),
        }
    }

    fn get_target_url(&self, route: &Route, original_path: &str) -> anyhow::Result<(String, u16, String)> {
        let target = Url::parse(&route.target)?;
        let stripped_path = original_path.strip_prefix(&route.context)
            .unwrap_or(original_path);
        let new_path = if stripped_path.is_empty() { "/" } else { stripped_path };
        
        let host = target.host_str()
            .ok_or_else(|| anyhow::anyhow!("No host in target URL"))?;
        let port = target.port().unwrap_or(if target.scheme() == "https" { 443 } else { 80 });
        
        Ok((host.to_string(), port, new_path.to_string()))
    }
}

pub struct MyCtx {
    buffer: Vec<u8>,
}

#[async_trait]
impl ProxyHttp for IdObfuscationProxy {
    type CTX = MyCtx;
    fn new_ctx(&self) -> Self::CTX {
        MyCtx { buffer: vec![] }
    }

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let path = session.req_header().uri.path();
        
        if let Some(route) = self.config.find_route(path) {
            let (host, port, new_path) = self.get_target_url(route, path)
                .map_err(|e| Error::because(ErrorType::InternalError, "failed to get target url", e))?;
            
            // Update the request path using set_uri instead of direct assignment
            let new_uri = new_path.parse().map_err(|e| Error::because(ErrorType::InternalError, "failed to set uri", e))?;
            session.req_header_mut().set_uri(new_uri);
            
            
            let peer = Box::new(HttpPeer::new(
                (host.as_str(), port),
                route.target.starts_with("https"),
                host.clone(),
            ));
            Ok(peer)
        } else {
            Err(Error::because(ErrorType::InternalError, "no matching route", anyhow::anyhow!("err")))
        }
    }

    fn response_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<Option<std::time::Duration>>
    where
        Self::CTX: Send + Sync,
    {
        // buffer the data
        if let Some(b) = body {
            ctx.buffer.extend(&b[..]);
            // drop the body
            b.clear();
        }
        if end_of_stream {
            // This is the last chunk, we can transform the body
            let processor = HtmlProcessor::new(self.id_mapping.clone());
            match processor.process(&ctx.buffer) {
                Ok(processed_html) => {
                    // Clear the buffer and replace with processed HTML
                    ctx.buffer.clear();
                    ctx.buffer.extend(processed_html);
                    *body = Some(Bytes::from(ctx.buffer.clone()));
                }
                Err(e) => {
                    
                    // On error, return the original content
                    *body = Some(Bytes::from(ctx.buffer.clone()));
                }
            }
        }
        Ok(None)
    }
}