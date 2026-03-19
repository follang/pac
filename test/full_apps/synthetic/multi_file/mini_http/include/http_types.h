#ifndef HTTP_TYPES_H
#define HTTP_TYPES_H

typedef enum HttpMethod {
    HTTP_GET = 1,
    HTTP_POST = 2
} HttpMethod;

typedef struct HttpRequest {
    HttpMethod method;
    const char *path;
    unsigned has_body : 1;
} HttpRequest;

#endif
