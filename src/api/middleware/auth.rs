//! 中间件：CORS、日志，以及鉴权扩展点。
//!
//! 归属：成员 B
//!
//! 说明：课程作业可简化为“无鉴权 + 放开 CORS”以便本地联调；
//! 若需实现登录，可在此模块内添加 `from_fn` 鉴权中间件，校验请求头 token。

use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// 放开跨域（前端本地调试用）
pub fn cors_layer() -> CorsLayer {
    CorsLayer::permissive()
}

/// HTTP 请求日志
pub fn trace_layer(
) -> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
}

// TODO(成员 B)：实现真正的鉴权中间件，例如：
// pub async fn require_auth(req: Request, next: Next) -> Result<Response, ApiError> { ... }
// 然后在 router 中 `.layer(from_fn(require_auth))`。
