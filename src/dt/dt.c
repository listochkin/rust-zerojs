#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "duktape.h"
#include "dt.h"

const void* create_vm() {
    return duk_create_heap_default();
};

char* eval(const void* vm, char* javascript) {
    int len = strlen(javascript);
    duk_eval_lstring(vm, javascript, len);
    char* result = duk_get_string(vm, -1);
    duk_pop(vm);
    return result;
};

void destroy_vm(const void* vm) {
    duk_destroy_heap(vm);
};
