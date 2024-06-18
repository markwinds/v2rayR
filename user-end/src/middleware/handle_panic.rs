// use std::panic;
//
// use actix_web::{Error, HttpRequest, HttpResponse, web};
// use actix_web::dev::{ServiceRequest, ServiceResponse};
//
// pub async fn handle_panic(req: ServiceRequest, srv: Box<dyn actix_web::dev::Service<ServiceRequest, Error=(), Future=(), Response=()>>) -> Result<ServiceResponse, Error> {
//     let res = panic::catch_unwind(|| srv.call(req));
//     match res {
//         Ok(fut) => {
//             let res = fut.await;
//             return res;
//         }
//         Err(err) => {
//             Ok(ServiceResponse::new(
//                 req.into_parts().0,
//                 HttpResponse::InternalServerError().json(ApiResponse::<()>::error(999, "Unknown error".into())).into(),
//             ))
//         }
//     }
// }
//
// // 定义一个简单的中间件函数
// async fn middleware_function(req: HttpRequest, next: web::types::Handler) -> HttpResponse {
//     println!("Middleware executed: {:?}", req);
//
//     // 在处理之前可以进行一些操作，例如记录请求信息等
//
//     // 调用下一个处理程序（可能是路由处理程序）
//     let response = next(req).await;
//
//     // 在处理之后可以进行一些操作，例如修改响应对象等
//
//     // 返回最终响应
//     response
// }
//
// // Define a simple middleware function
// async fn middleware_function1(
//     req: HttpRequest,
//     service: web::Service,
// ) -> Result<HttpResponse, actix_web::Error> {
//     println!("Middleware executed: {:?}", req);
//
//     // Before handler
//     let response = service(req).await;
//
//     // After handler
//     println!("Middleware response: {:?}", response);
//
//     // Return the response
//     Ok(response)
// }


// use std::future::{Ready, ready};
// use std::panic;
//
// use actix_web::{Error, HttpResponse};
// use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
// use futures_util::future::LocalBoxFuture;
//
// pub struct HandlePanic;
//
// impl<S, B> Transform<S, ServiceRequest> for HandlePanic
//     where
//         S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
//         S::Future: 'static,
//         B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Transform = HandlePanicMiddleware<S>;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(HandlePanicMiddleware { service }))
//     }
// }
//
// pub struct HandlePanicMiddleware<S> {
//     service: S,
// }
//
// impl<S, B> Service<ServiceRequest> for HandlePanicMiddleware<S>
//     where
//         S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
//         S::Future: 'static,
//         B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//     forward_ready!(service);
//
//     // 中间件主要就改这里 其他都是模板代码
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let a = req.into_parts().clone();
//
//         // 包裹捕获panic信息
//         let res = panic::catch_unwind(|| self.service.call(req));
//         match res {
//             Ok(fut) => {
//                 Box::pin(async move {
//                     let res = fut.await?;
//                     Ok(res)
//                 })
//             }
//             Err(e) => {
//                 Box::pin(async move {
//                     // let res = fut.await?;
//                     // Ok(res)
//                     Ok(ServiceResponse::new(a, HttpResponse::Ok().json("good")))
//                 })
//             }
//         }
//     }
// }



