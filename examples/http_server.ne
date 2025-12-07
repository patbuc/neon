// Example HTTP server in Neon

fn handleRoot(request) {
    print("Handling request to " + request.path());
    return "Hello from Neon HTTP Server!";
}

fn handleEcho(request) {
    let method = request.method();
    let path = request.path();
    let body = request.body();

    return "Method: " + method + "\nPath: " + path + "\nBody: " + body;
}

let server = HttpServer(8080);
server.on("/", handleRoot);
server.on("/echo", handleEcho);

print("Starting server...");
server.start();
