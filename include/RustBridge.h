#ifndef RustBridge_h
#define RustBridge_h

#include <stdio.h>

const char* theme(const char* password, const char* path_cartes, const char* nom, const char* date, int id);
const char* selection_traitment(const char* password, int type_traitement, int id, int carte);
char* free_cstring(ptr: char*);
const char* cyclesPng(int j, int m, int a, int age, const char* path_cycle);

#endif /* RustBridge_h */