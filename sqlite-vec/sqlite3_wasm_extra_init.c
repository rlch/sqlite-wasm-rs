/*
** Auto-initialization for sqlite-vec extension in WASM builds
** 
** This file is automatically detected by the build process when present.
** It registers sqlite-vec using sqlite3_auto_extension() so that the
** extension is available immediately when SQLite initializes.
*/

#include "sqlite3.h"

// Forward declare the init function from sqlite-vec.c
extern int sqlite3_vec_init(sqlite3 *db, char **pzErrMsg, const sqlite3_api_routines *pApi);

/*
** This function is called during sqlite3_initialize().
** It registers sqlite-vec to be automatically loaded for all database connections.
*/
/*
** WASM builds cannot use sqlite3_auto_extension due to function pointer limitations.
** Instead, sqlite3_vec_init() should be called manually for each database connection.
**
** For now, this just returns OK. Users must call sqlite3_vec_init(db, NULL, NULL)
** after opening each database to enable sqlite-vec functions.
*/
int sqlite3_wasm_extra_init(const char *z){
  (void)z;  // Unused parameter
  // sqlite3_auto_extension doesn't work in WASM
  // Users must manually call: sqlite3_vec_init(db, NULL, NULL)
  return SQLITE_OK;
}
