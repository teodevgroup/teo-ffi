#include "example.h"
#include <stdio.h>
#include <unistd.h>

void app_callback(App app) {
    App_Run(&app);
}

Value my(Value value) {
    char *now = Value_IntoString(value);
    char *after = "abcqqoqxx@gmaaa.com";
    return Value_FromString(after);
}

int main(void) {
    AppBuilder app_builder = AppBuilder_Create("C", "Apple clang 14.0.0");
    AppBuilder_Transform(&app_builder, "my", my);
    AppBuilder_Load(&app_builder, "schema.teo");
    AppBuilder_Build(&app_builder, &app_callback);
    sleep(5000);
    return 0;
}
