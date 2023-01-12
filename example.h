#ifndef TEOFFI_H
#define TEOFFI_H

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

extern int Value_IsNull(Value value);
extern char *Value_IntoString(Value value);
extern void Value_FreeString(char *str);
extern int Value_IntoBool(Value value);
extern char Value_IntoChar(Value value);
extern short Value_IntoShort(Value value);
extern int Value_IntoInt(Value value);
extern long Value_IntoLong(Value value);
extern long long Value_IntoLongLong(Value value);
extern unsigned char Value_IntoUChar(Value value);
extern unsigned short Value_IntoUShort(Value value);
extern unsigned int Value_IntoUInt(Value value);
extern unsigned long Value_IntoULong(Value value);
extern unsigned long long Value_IntoULongLong(Value value);
extern Value Value_FromNull();
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
typedef Value (*ValueTransform) (Value value);
typedef Value (*ValueValidator) (Value value);
typedef void (*ValuePerformer) (Value value);

extern AppBuilder AppBuilder_Create(const char *lang, const char *version);
extern void AppBuilder_Transform(AppBuilder *app_builder, char *name, ValueTransform transform);
extern void AppBuilder_Validate(AppBuilder *app_builder, char *name, ValueValidator validator);
extern void AppBuilder_Perform(AppBuilder *app_builder, char *name, ValuePerformer performer);
extern void AppBuilder_Build(AppBuilder *app_builder);

#endif
