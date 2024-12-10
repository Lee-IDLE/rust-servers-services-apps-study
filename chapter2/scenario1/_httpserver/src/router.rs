use super::handler::{Handler, PageNotFoundHandler, StatitcPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, HttpResponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // 해당 URI를 파싱한다
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] { 
                        // 경로가 /api로 시작하면 Web 서비스를 호출한다
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // 그렇지 않면 정적 페이지 핸들러를 호출한다
                        _ => {
                            let resp: HttpResponse = StatitcPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // 메서드가 GET 요청이 아니면 404 페이지를 반환한다.
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            },
        }
    }
}