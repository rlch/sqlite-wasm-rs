use sqlite_wasm_rs::{sahpool_vfs::OpfsSAHPoolCfgBuilder, *};
use std::ffi::CString;
use wasm_bindgen_test::wasm_bindgen_test;

// External function from sqlite-vec.c
extern "C" {
    fn sqlite3_vec_init(
        db: *mut sqlite3,
        pzErrMsg: *mut *mut std::os::raw::c_char,
        pApi: *const std::os::raw::c_void,
    ) -> std::os::raw::c_int;
}

// Helper to initialize sqlite-vec on a database connection
unsafe fn init_sqlite_vec(db: *mut sqlite3) {
    let ret = sqlite3_vec_init(db, std::ptr::null_mut(), std::ptr::null());
    assert_eq!(SQLITE_OK, ret, "Failed to initialize sqlite-vec");
}

unsafe fn prepare_vec_table(db: *mut sqlite3) {
    // Create a vec0 virtual table for vector search
    let sql = c"
CREATE VIRTUAL TABLE vec_items USING vec0(
    embedding float[3]
);

INSERT INTO vec_items(rowid, embedding) VALUES 
    (1, '[1.0, 2.0, 3.0]'),
    (2, '[4.0, 5.0, 6.0]'),
    (3, '[7.0, 8.0, 9.0]');
    ";
    let ret = sqlite3_exec(
        db,
        sql.as_ptr().cast(),
        None,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    assert_eq!(ret, SQLITE_OK);
}

unsafe fn check_vec_version(db: *mut sqlite3) {
    // Check that vec_version() function is available
    let sql = c"SELECT vec_version();";
    let mut stmt = std::ptr::null_mut();
    let ret = sqlite3_prepare_v3(
        db,
        sql.as_ptr().cast(),
        -1,
        0,
        &mut stmt as *mut _,
        std::ptr::null_mut(),
    );
    assert_eq!(ret, SQLITE_OK);

    let ret = sqlite3_step(stmt);
    assert_eq!(ret, SQLITE_ROW);

    // Just verify we got a text result (the version string)
    let ty = sqlite3_column_type(stmt, 0);
    assert_eq!(ty, SQLITE_TEXT);

    sqlite3_finalize(stmt);
}

unsafe fn check_vec_search(db: *mut sqlite3) {
    // Perform a KNN search
    let sql = c"
SELECT rowid, distance 
FROM vec_items 
WHERE embedding MATCH '[1.5, 2.5, 3.5]'
ORDER BY distance
LIMIT 2;
    ";
    let mut stmt = std::ptr::null_mut();
    let ret = sqlite3_prepare_v3(
        db,
        sql.as_ptr().cast(),
        -1,
        0,
        &mut stmt as *mut _,
        std::ptr::null_mut(),
    );
    assert_eq!(ret, SQLITE_OK);

    // Should get 2 results
    let mut count = 0;
    while sqlite3_step(stmt) == SQLITE_ROW {
        // First column is rowid (integer)
        let ty = sqlite3_column_type(stmt, 0);
        assert_eq!(ty, SQLITE_INTEGER);
        
        // Second column is distance (float)
        let ty = sqlite3_column_type(stmt, 1);
        assert_eq!(ty, SQLITE_FLOAT);
        
        count += 1;
    }
    assert_eq!(count, 2);

    sqlite3_finalize(stmt);
}

unsafe fn test_memvfs_vec() {
    let mut db = std::ptr::null_mut();
    let db_name = "test_memvfs_vec.db";

    let c_name = CString::new(db_name).unwrap();
    let ret = sqlite3_open_v2(
        c_name.as_ptr().cast(),
        &mut db as *mut _,
        SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
        std::ptr::null(),
    );
    assert_eq!(SQLITE_OK, ret);

    // Manually initialize sqlite-vec for this connection
    init_sqlite_vec(db);

    check_vec_version(db);
    prepare_vec_table(db);
    check_vec_search(db);
    
    let ret = sqlite3_close(db);
    assert_eq!(ret, SQLITE_OK);
}

async unsafe fn test_opfs_sah_vfs_vec() {
    let _util = sqlite_wasm_rs::sahpool_vfs::install(
        &OpfsSAHPoolCfgBuilder::new()
            .vfs_name("sah-vec")
            .directory("sah-vec")
            .initial_capacity(20)
            .clear_on_init(true)
            .build(),
        false,
    )
    .await
    .unwrap();

    let mut db = std::ptr::null_mut();
    let db_name = "test_opfs_sah_vfs_vec.db";

    let c_name = CString::new(db_name).unwrap();
    let ret = sqlite3_open_v2(
        c_name.as_ptr().cast(),
        &mut db as *mut _,
        SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
        c"sah-vec".as_ptr().cast(),
    );
    assert_eq!(SQLITE_OK, ret);

    // Manually initialize sqlite-vec
    init_sqlite_vec(db);

    check_vec_version(db);
    prepare_vec_table(db);
    check_vec_search(db);
    
    let ret = sqlite3_close(db);
    assert_eq!(ret, SQLITE_OK);
}

#[cfg(feature = "relaxed-idb")]
async unsafe fn test_relaxed_idb_vfs_vec() {
    let _util = sqlite_wasm_rs::relaxed_idb_vfs::install(
        &sqlite_wasm_rs::relaxed_idb_vfs::RelaxedIdbCfgBuilder::new()
            .vfs_name("relaxed-db-vec")
            .clear_on_init(true)
            .build(),
        false,
    )
    .await
    .unwrap();

    let mut db = std::ptr::null_mut();
    let db_name = "test_relaxed_db_vfs_vec.db";

    let c_name = CString::new(db_name).unwrap();
    let ret = sqlite3_open_v2(
        c_name.as_ptr().cast(),
        &mut db as *mut _,
        SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
        c"relaxed-db-vec".as_ptr().cast(),
    );
    assert_eq!(SQLITE_OK, ret);

    // Manually initialize sqlite-vec
    init_sqlite_vec(db);

    check_vec_version(db);
    prepare_vec_table(db);
    check_vec_search(db);
    
    let ret = sqlite3_close(db);
    assert_eq!(ret, SQLITE_OK);
}

#[wasm_bindgen_test]
fn test_memvfs_sqlite_vec() {
    unsafe {
        test_memvfs_vec();
    }
}

#[wasm_bindgen_test]
async fn test_opfs_sah_vfs_sqlite_vec() {
    unsafe {
        test_opfs_sah_vfs_vec().await;
    }
}

#[cfg(feature = "relaxed-idb")]
#[wasm_bindgen_test]
async fn test_relaxed_idb_vfs_sqlite_vec() {
    unsafe {
        test_relaxed_idb_vfs_vec().await;
    }
}
