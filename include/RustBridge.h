#ifndef RustBridge_h
#define RustBridge_h

#include <stdio.h>

const char* theme(const char* password, const char* path_cartes, const char* nom, const char* date, int id);
const char* selection_traitment(const char* password, int type_traitement, int id);
char* free_cstring(ptr: char*);

#endif /* RustBridge_h */