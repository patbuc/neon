use crate::common::{Object, Value, ObjString, ObjHttpServer, ObjHttpRequest};
use crate::vm::VirtualMachine;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;

pub fn native_http_server_constructor(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("HttpServer() expects 1 argument (port), got {}", args.len()));
    }

    let port = match &args[0] {
        Value::Number(n) => {
            if *n < 1.0 || *n > 65535.0 || n.fract() != 0.0 {
                return Err("Port must be an integer between 1 and 65535".to_string());
            }
            *n as u16
        }
        _ => return Err("HttpServer() requires a number argument".to_string()),
    };

    let server_obj = ObjHttpServer {
        port,
        routes: HashMap::new(),
    };

    Ok(Value::Object(Rc::new(Object::HttpServer(
        Rc::new(RefCell::new(server_obj))
    ))))
}

pub fn native_http_server_on(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!("on() expects 2 arguments (path, handler), got {}", args.len() - 1));
    }

    let server_ref = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::HttpServer(srv) => srv,
            _ => return Err("on() can only be called on HttpServer".to_string()),
        },
        _ => return Err("on() can only be called on HttpServer".to_string()),
    };

    let path = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.to_string(),
            _ => return Err("on() requires a string path".to_string()),
        },
        _ => return Err("on() requires a string path".to_string()),
    };

    if !path.starts_with('/') {
        return Err("Path must start with '/'".to_string());
    }

    let handler = match &args[2] {
        Value::Object(obj) => match obj.as_ref() {
            Object::Function(_) | Object::NativeFunction(_) => args[2].clone(),
            _ => return Err("on() requires a function handler".to_string()),
        },
        _ => return Err("on() requires a function handler".to_string()),
    };

    let mut server = server_ref.borrow_mut();
    if server.routes.contains_key(&path) {
        return Err(format!("Route '{}' is already registered", path));
    }
    server.routes.insert(path, handler);

    Ok(Value::Nil)
}

pub fn native_http_server_start(
    vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("start() expects no arguments".to_string());
    }

    // Extract port and routes
    let (port, routes) = {
        let server_ref = match &args[0] {
            Value::Object(obj) => match obj.as_ref() {
                Object::HttpServer(srv) => srv,
                _ => return Err("start() can only be called on HttpServer".to_string()),
            },
            _ => return Err("start() can only be called on HttpServer".to_string()),
        };

        let server = server_ref.borrow();
        if server.routes.is_empty() {
            return Err("Cannot start server: no routes registered".to_string());
        }

        (server.port, server.routes.clone())
    };

    // Create tiny_http server
    let http_server = match tiny_http::Server::http(("0.0.0.0", port)) {
        Ok(srv) => srv,
        Err(e) => return Err(format!("Failed to start server on port {}: {}", port, e)),
    };

    println!("Server listening on http://0.0.0.0:{}", port);

    // BLOCKING LOOP - handle incoming requests
    for mut request in http_server.incoming_requests() {
        let method = request.method().to_string();
        let path = request.url().to_string();

        // Read request body
        let body = {
            let mut buf = String::new();
            if let Some(len) = request.body_length() {
                let reader = request.as_reader();
                let mut limited = reader.take(len as u64);
                let _ = limited.read_to_string(&mut buf);
            }
            buf
        };

        // Extract headers
        let mut headers = HashMap::new();
        for header in request.headers() {
            headers.insert(
                header.field.to_string(),
                header.value.to_string()
            );
        }

        // Find matching route
        let handler = match routes.get(&path) {
            Some(h) => h.clone(),
            None => {
                let response = tiny_http::Response::from_string("Not Found")
                    .with_status_code(404);
                let _ = request.respond(response);
                continue;
            }
        };

        // Create HttpRequest object
        let req_obj = ObjHttpRequest {
            method,
            path: path.clone(),
            body,
            headers,
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req_obj))));

        // Call handler function
        let result = vm.call_function_with_args(&handler, vec![request_value]);

        // Handle result
        let response_body = match result {
            Ok(Value::Object(obj)) => match obj.as_ref() {
                Object::String(s) => s.value.to_string(),
                _ => "OK".to_string(),
            },
            Ok(_) => "OK".to_string(),
            Err(e) => {
                let response = tiny_http::Response::from_string(
                    format!("Internal Server Error: {}", e)
                ).with_status_code(500);
                let _ = request.respond(response);
                continue;
            }
        };

        // Send response
        let response = tiny_http::Response::from_string(response_body)
            .with_status_code(200);
        let _ = request.respond(response);
    }

    Ok(Value::Nil)
}

pub fn native_http_request_method(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("method() expects no arguments".to_string());
    }

    let request = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::HttpRequest(req) => req,
            _ => return Err("method() can only be called on HttpRequest".to_string()),
        },
        _ => return Err("method() can only be called on HttpRequest".to_string()),
    };

    Ok(Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(request.method.as_str()),
    }))))
}

pub fn native_http_request_path(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("path() expects no arguments".to_string());
    }

    let request = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::HttpRequest(req) => req,
            _ => return Err("path() can only be called on HttpRequest".to_string()),
        },
        _ => return Err("path() can only be called on HttpRequest".to_string()),
    };

    Ok(Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(request.path.as_str()),
    }))))
}

pub fn native_http_request_body(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("body() expects no arguments".to_string());
    }

    let request = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::HttpRequest(req) => req,
            _ => return Err("body() can only be called on HttpRequest".to_string()),
        },
        _ => return Err("body() can only be called on HttpRequest".to_string()),
    };

    Ok(Value::Object(Rc::new(Object::String(ObjString {
        value: Rc::from(request.body.as_str()),
    }))))
}

pub fn native_http_request_header(
    _vm: &mut VirtualMachine,
    args: &[Value]
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("header() expects 1 argument (name), got {}", args.len() - 1));
    }

    let request = match &args[0] {
        Value::Object(obj) => match obj.as_ref() {
            Object::HttpRequest(req) => req,
            _ => return Err("header() can only be called on HttpRequest".to_string()),
        },
        _ => return Err("header() can only be called on HttpRequest".to_string()),
    };

    let header_name = match &args[1] {
        Value::Object(obj) => match obj.as_ref() {
            Object::String(s) => s.value.to_string(),
            _ => return Err("header() requires a string argument".to_string()),
        },
        _ => return Err("header() requires a string argument".to_string()),
    };

    match request.headers.get(&header_name) {
        Some(value) => Ok(Value::Object(Rc::new(Object::String(ObjString {
            value: Rc::from(value.as_str()),
        })))),
        None => Ok(Value::Nil),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VirtualMachine;

    #[test]
    fn test_http_server_constructor_valid() {
        let mut vm = VirtualMachine::new();
        let port = Value::Number(8080.0);
        let result = native_http_server_constructor(&mut vm, &[port]);
        assert!(result.is_ok());

        if let Ok(Value::Object(obj)) = result {
            if let Object::HttpServer(srv) = obj.as_ref() {
                assert_eq!(srv.borrow().port, 8080);
                assert!(srv.borrow().routes.is_empty());
            } else {
                panic!("Expected HttpServer object");
            }
        }
    }

    #[test]
    fn test_http_server_constructor_invalid_port_negative() {
        let mut vm = VirtualMachine::new();
        let port = Value::Number(-1.0);
        let result = native_http_server_constructor(&mut vm, &[port]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 1 and 65535"));
    }

    #[test]
    fn test_http_server_constructor_invalid_port_too_large() {
        let mut vm = VirtualMachine::new();
        let port = Value::Number(70000.0);
        let result = native_http_server_constructor(&mut vm, &[port]);
        assert!(result.is_err());
    }

    #[test]
    fn test_http_server_constructor_invalid_port_float() {
        let mut vm = VirtualMachine::new();
        let port = Value::Number(8080.5);
        let result = native_http_server_constructor(&mut vm, &[port]);
        assert!(result.is_err());
    }

    #[test]
    fn test_http_request_method() {
        let mut vm = VirtualMachine::new();
        let req = ObjHttpRequest {
            method: "GET".to_string(),
            path: "/test".to_string(),
            body: String::new(),
            headers: HashMap::new(),
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req))));

        let result = native_http_request_method(&mut vm, &[request_value]);
        assert!(result.is_ok());

        if let Ok(Value::Object(obj)) = result {
            if let Object::String(s) = obj.as_ref() {
                assert_eq!(s.value.as_ref(), "GET");
            } else {
                panic!("Expected String object");
            }
        }
    }

    #[test]
    fn test_http_request_path() {
        let mut vm = VirtualMachine::new();
        let req = ObjHttpRequest {
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            body: String::new(),
            headers: HashMap::new(),
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req))));

        let result = native_http_request_path(&mut vm, &[request_value]);
        assert!(result.is_ok());

        if let Ok(Value::Object(obj)) = result {
            if let Object::String(s) = obj.as_ref() {
                assert_eq!(s.value.as_ref(), "/api/users");
            }
        }
    }

    #[test]
    fn test_http_request_body() {
        let mut vm = VirtualMachine::new();
        let req = ObjHttpRequest {
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            body: "{\"name\":\"Alice\"}".to_string(),
            headers: HashMap::new(),
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req))));

        let result = native_http_request_body(&mut vm, &[request_value]);
        assert!(result.is_ok());

        if let Ok(Value::Object(obj)) = result {
            if let Object::String(s) = obj.as_ref() {
                assert_eq!(s.value.as_ref(), "{\"name\":\"Alice\"}");
            }
        }
    }

    #[test]
    fn test_http_request_header_exists() {
        let mut vm = VirtualMachine::new();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let req = ObjHttpRequest {
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            body: String::new(),
            headers,
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req))));
        let header_name = Value::Object(Rc::new(Object::String(crate::common::ObjString {
            value: Rc::from("Content-Type"),
        })));

        let result = native_http_request_header(&mut vm, &[request_value, header_name]);
        assert!(result.is_ok());

        if let Ok(Value::Object(obj)) = result {
            if let Object::String(s) = obj.as_ref() {
                assert_eq!(s.value.as_ref(), "application/json");
            }
        }
    }

    #[test]
    fn test_http_request_header_not_exists() {
        let mut vm = VirtualMachine::new();
        let req = ObjHttpRequest {
            method: "GET".to_string(),
            path: "/".to_string(),
            body: String::new(),
            headers: HashMap::new(),
        };
        let request_value = Value::Object(Rc::new(Object::HttpRequest(Rc::new(req))));
        let header_name = Value::Object(Rc::new(Object::String(crate::common::ObjString {
            value: Rc::from("X-Custom-Header"),
        })));

        let result = native_http_request_header(&mut vm, &[request_value, header_name]);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Value::Nil));
    }
}
