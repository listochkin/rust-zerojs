// #include "duktape.h"

// int main(int argc, char *argv[]) {
//   duk_context *ctx = duk_create_heap_default();
//   duk_eval_string(ctx, "print('Hello world!');");
//   duk_destroy_heap(ctx);
//   return 0;
// }

const void* create_vm();
char* eval(const void* vm, char* javascript);
void destroy_vm(const void* vm);
