#ifndef MINI_HTTP_H
#define MINI_HTTP_H

#include "http_types.h"

int http_request_valid(const HttpRequest *request);
HttpRequest http_make_request(HttpMethod method, const char *path, unsigned has_body);

#endif
