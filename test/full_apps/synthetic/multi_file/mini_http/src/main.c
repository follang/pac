#include "../include/mini_http.h"

static int path_is_empty(const char *path) {
    return path == (const char *)0 || path[0] == '\0';
}

int http_request_valid(const HttpRequest *request) {
    if (request == (const HttpRequest *)0) {
        return 0;
    }
    if (path_is_empty(request->path)) {
        return 0;
    }
    if (request->method == HTTP_POST && !request->has_body) {
        return 0;
    }
    return 1;
}

HttpRequest http_make_request(HttpMethod method, const char *path, unsigned has_body) {
    HttpRequest request = { method, path, has_body ? 1u : 0u };
    return request;
}

int main(void) {
    HttpRequest request = http_make_request(HTTP_GET, "/status", 0u);
    return http_request_valid(&request) ? 0 : 1;
}
