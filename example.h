#include "stddef.h"

typedef struct _Value {
    size_t _arc_teo_value;
} Value;

typedef struct _AppBuilder {
    size_t _arc_teo_app_builder;
} AppBuilder;

typedef struct _App {
    size_t _arc_teo_app;
} App;

extern Value Value_FromString(const char *str);
extern Value Value_BoolFromInt(int b);
extern Value Value_FromChar(char i);
extern Value Value_FromShort(short i);
extern Value Value_FromInt(int i);
extern Value Value_FromLong(long i);
extern Value Value_FromLongLong(long long i);
extern Value Value_FromUChar(unsigned char i);
extern Value Value_FromUShort(unsigned short i);
extern Value Value_FromUInt(unsigned int i);
extern Value Value_FromULong(unsigned long i);
extern Value Value_FromULongLong(unsigned long long i);

typedef void (*AppCallback) (App app);

extern AppBuilder AppBuilder_Create(const char *lang, const char *version);
extern void AppBuilder_Load(AppBuilder *app_builder, char *schema_file_name);
extern void AppBuilder_Build(AppBuilder *app_builder, AppCallback app_callback);
extern void App_Run(App *app);
