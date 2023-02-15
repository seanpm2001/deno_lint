// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.
use super::Context;
use super::LintRule;
use crate::handler::Handler;
use crate::handler::Traverse;
use crate::Program;

use deno_ast::view as ast_view;
use deno_ast::SourceRanged;
use if_chain::if_chain;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct NoWindowPrefix;

const CODE: &str = "no-window-prefix";
const MESSAGE: &str = "For compatibility between the Window context and the Web Workers, calling Web APIs via `window` is disallowed";
const HINT: &str =
  "Instead, call this API via `self`, `globalThis`, or no extra prefix";

impl LintRule for NoWindowPrefix {
  fn new() -> Arc<Self> {
    Arc::new(NoWindowPrefix)
  }

  fn tags(&self) -> &'static [&'static str] {
    &["recommended"]
  }

  fn code(&self) -> &'static str {
    CODE
  }

  fn lint_program_with_ast_view(
    &self,
    context: &mut Context,
    program: Program<'_>,
  ) {
    NoWindowPrefixHandler.traverse(program, context);
  }

  #[cfg(feature = "docs")]
  fn docs(&self) -> &'static str {
    include_str!("../../docs/rules/no_window_prefix.md")
  }
}

// Auto-generated by tools/generate_no_window_prefix_deny_list.ts
static PROPERTY_DENY_LIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
  [
    "AbortController",
    "AbortSignal",
    "Blob",
    "BroadcastChannel",
    "ByteLengthQueuingStrategy",
    "Cache",
    "CacheStorage",
    "CanvasGradient",
    "CanvasPattern",
    "CloseEvent",
    "CountQueuingStrategy",
    "Crypto",
    "CryptoKey",
    "CustomEvent",
    "DOMException",
    "DOMMatrix",
    "DOMMatrixReadOnly",
    "DOMPoint",
    "DOMPointReadOnly",
    "DOMQuad",
    "DOMRect",
    "DOMRectReadOnly",
    "DOMStringList",
    "ErrorEvent",
    "Event",
    "EventSource",
    "EventTarget",
    "File",
    "FileList",
    "FileReader",
    "FontFace",
    "FontFaceSet",
    "FontFaceSetLoadEvent",
    "FormData",
    "Headers",
    "IDBCursor",
    "IDBCursorWithValue",
    "IDBDatabase",
    "IDBFactory",
    "IDBIndex",
    "IDBKeyRange",
    "IDBObjectStore",
    "IDBOpenDBRequest",
    "IDBRequest",
    "IDBTransaction",
    "IDBVersionChangeEvent",
    "ImageBitmap",
    "ImageBitmapRenderingContext",
    "ImageData",
    "MediaCapabilities",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "NetworkInformation",
    "Notification",
    "Path2D",
    "Performance",
    "PerformanceEntry",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformanceResourceTiming",
    "PerformanceServerTiming",
    "PermissionStatus",
    "Permissions",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PushManager",
    "PushSubscription",
    "PushSubscriptionOptions",
    "ReadableStream",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "Request",
    "Response",
    "SecurityPolicyViolationEvent",
    "ServiceWorker",
    "ServiceWorkerContainer",
    "ServiceWorkerRegistration",
    "StorageManager",
    "SubtleCrypto",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TextMetrics",
    "TransformStream",
    "TransformStreamDefaultController",
    "URL",
    "URLSearchParams",
    "WebGL2RenderingContext",
    "WebGLActiveInfo",
    "WebGLBuffer",
    "WebGLContextEvent",
    "WebGLFramebuffer",
    "WebGLProgram",
    "WebGLQuery",
    "WebGLRenderbuffer",
    "WebGLRenderingContext",
    "WebGLSampler",
    "WebGLShader",
    "WebGLShaderPrecisionFormat",
    "WebGLSync",
    "WebGLTexture",
    "WebGLTransformFeedback",
    "WebGLUniformLocation",
    "WebGLVertexArrayObject",
    "WebSocket",
    "Worker",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "XMLHttpRequest",
    "XMLHttpRequestEventTarget",
    "XMLHttpRequestUpload",
    "console",
    "WebAssembly",
    "name",
    "navigator",
    "self",
    "close",
    "postMessage",
    "dispatchEvent",
    "cancelAnimationFrame",
    "requestAnimationFrame",
    "onerror",
    "onlanguagechange",
    "onmessage",
    "onmessageerror",
    "onoffline",
    "ononline",
    "onrejectionhandled",
    "onunhandledrejection",
    "caches",
    "crossOriginIsolated",
    "crypto",
    "indexedDB",
    "isSecureContext",
    "origin",
    "performance",
    "atob",
    "btoa",
    "clearInterval",
    "clearTimeout",
    "createImageBitmap",
    "fetch",
    "queueMicrotask",
    "setInterval",
    "setTimeout",
    "addEventListener",
    "removeEventListener",
    "Deno",
  ]
  .iter()
  .copied()
  .collect()
});

/// Extracts a symbol from the given expression if the symbol is statically determined (otherwise,
/// return `None`).
fn extract_symbol<'a>(expr: &'a ast_view::MemberExpr) -> Option<&'a str> {
  use deno_ast::view::{Expr, Lit, MemberProp, Tpl};
  match &expr.prop {
    MemberProp::Ident(ident) => Some(ident.sym()),
    MemberProp::PrivateName(name) => Some(name.id.sym()),
    MemberProp::Computed(prop) => match &prop.expr {
      Expr::Lit(Lit::Str(s)) => Some(s.value()),
      // If it's computed, this MemberExpr looks like `foo[bar]`
      Expr::Ident(_) => None,
      Expr::Tpl(Tpl {
        ref exprs,
        ref quasis,
        ..
      }) if exprs.is_empty() && quasis.len() == 1 => Some(quasis[0].raw()),
      _ => None,
    },
  }
}

struct NoWindowPrefixHandler;

impl Handler for NoWindowPrefixHandler {
  fn member_expr(
    &mut self,
    member_expr: &ast_view::MemberExpr,
    ctx: &mut Context,
  ) {
    // Don't check chained member expressions (e.g. `foo.bar.baz`)
    if member_expr.parent().is::<ast_view::MemberExpr>() {
      return;
    }

    use deno_ast::view::Expr;
    if_chain! {
      if let Expr::Ident(obj) = &member_expr.obj;
      let obj_symbol = obj.sym();
      if obj_symbol == "window";
      if ctx.scope().is_global(&obj.inner.to_id());
      if let Some(prop_symbol) = extract_symbol(member_expr);
      if PROPERTY_DENY_LIST.contains(prop_symbol);
      then {
        ctx.add_diagnostic_with_hint(
          member_expr.range(),
          CODE,
          MESSAGE,
          HINT,
        );
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_deprecated_deno_api_valid() {
    assert_lint_ok! {
      NoWindowPrefix,
      "fetch();",
      "self.fetch();",
      "globalThis.fetch();",

      "Deno.metrics();",
      "self.Deno.metrics();",
      "globalThis.Deno.metrics();",

      "onload();",
      "self.onload();",
      "globalThis.onload();",
      "window.onload();",
      r#"window["onload"]();"#,
      r#"window[`onload`]();"#,

      "onunload();",
      "self.onunload();",
      "globalThis.onunload();",
      "window.onunload();",
      r#"window["onunload"]();"#,
      r#"window[`onunload`]();"#,

      "closed;",
      "self.closed;",
      "globalThis.closed;",
      "window.closed;",
      r#"window["closed"];"#,
      r#"window[`closed`];"#,

      "alert();",
      "self.alert();",
      "globalThis.alert();",
      "window.alert();",
      r#"window["alert"]();"#,
      r#"window[`alert`]();"#,

      "confirm();",
      "self.confirm();",
      "globalThis.confirm();",
      "window.confirm();",
      r#"window["confirm"]();"#,
      r#"window[`confirm`]();"#,

      "prompt();",
      "self.prompt();",
      "globalThis.prompt();",
      "window.prompt();",
      r#"window["prompt"]();"#,
      r#"window[`prompt`]();"#,

      "localStorage;",
      "self.localStorage;",
      "globalThis.localStorage;",
      "window.localStorage;",
      r#"window["localStorage"];"#,
      r#"window[`localStorage`];"#,

      "sessionStorage;",
      "self.sessionStorage;",
      "globalThis.sessionStorage;",
      "window.sessionStorage;",
      r#"window["sessionStorage"];"#,
      r#"window[`sessionStorage`];"#,

      "window;",
      "self.window;",
      "globalThis.window;",
      "window.window;",
      r#"window["window"];"#,
      r#"window[`window`];"#,

      "Navigator;",
      "self.Navigator;",
      "globalThis.Navigator;",
      "window.Navigator;",
      r#"window["Navigator"];"#,
      r#"window[`Navigator`];"#,

      "location;",
      "self.location;",
      "globalThis.location;",
      "window.location;",
      r#"window["location"];"#,
      r#"window[`location`];"#,

      "history;",
      "self.history;",
      "globalThis.history;",
      "window.history;",
      r#"window["history"];"#,
      r#"window[`history`];"#,

      // `window` is shadowed
      "const window = 42; window.fetch();",
      r#"const window = 42; window["fetch"]();"#,
      r#"const window = 42; window[`fetch`]();"#,
      "const window = 42; window.alert();",
      r#"const window = 42; window["alert"]();"#,
      r#"const window = 42; window[`alert`]();"#,

      // Ignore property access with variables
      r#"const f = "fetch"; window[f]();"#,
      r#"const f = "fetch"; window[`${f}`]();"#,

      // Make sure that no false positives are triggered on chained member
      // expressions
      r#"foo.window.fetch();"#,
    };
  }

  #[test]
  fn no_deprecated_deno_api_invalid() {
    assert_lint_err! {
      NoWindowPrefix,
      MESSAGE,
      HINT,
      r#"window.fetch()"#: [
        {
          col: 0,
        }
      ],
      r#"window["fetch"]()"#: [
        {
          col: 0,
        }
      ],
      r#"window[`fetch`]()"#: [
        {
          col: 0,
        }
      ],
      r#"
function foo() {
  const window = 42;
  return window;
}
window.fetch();
      "#: [
        {
          col: 0,
          line: 6,
        }
      ],
    };
  }
}
