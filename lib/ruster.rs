//use std::sync::{Arc, Mutex};
//use std::error::Error as StdError;
//use std::task;
//use std::thread;
//use std::time;
//use std::io::{self, Write};
extern crate libc;
use std::ffi::CStr;
use std::process::Command;

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("MYSQL_ROOT_PASSWORD");
/// C.mysqldump_all_databases(
/// 	C.CString("localhost"),
/// 	C.CString("3306"),
/// 	C.CString("root"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn mysqldump_all_databases(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    filename_pattern: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_filename_pattern = unsafe { CStr::from_ptr(filename_pattern).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("mysqldump")
            .stdout(std::process::Stdio::piped())
            .arg("--verbose")
            .arg("--host")
            .arg("--single-transaction")
            .arg("--quick")
            .arg("--lock-tables=false")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--result-file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .arg("--all-databases")
            .spawn()
            .expect("mysqldump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysqldump_all_databases(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("mysqldump")
            .stdout(std::process::Stdio::piped())
            .arg("--single-transaction")
            .arg("--quick")
            .arg("--lock-tables=false")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--result-file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .arg("--all-databases")
            .spawn()
            .expect("mysqldump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysqldump_all_databases(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("MYSQL_ROOT_PASSWORD");
/// C.mysqldump_database(
/// 	C.CString("localhost"),
/// 	C.CString("3306"),
/// 	C.CString("root"),
/// 	C.CString("mysql"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn mysqldump_database(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    database: *const libc::c_char,
    filename_pattern: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_database = unsafe { CStr::from_ptr(database).to_bytes() };
    let buf_filename_pattern = unsafe { CStr::from_ptr(filename_pattern).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("mysqldump")
            .stdout(std::process::Stdio::piped())
            .arg("--verbose")
            .arg("--single-transaction")
            .arg("--quick")
            .arg("--lock-tables=false")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--databases")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--result-file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysqldump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysqldump_database(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("mysqldump")
            .stdout(std::process::Stdio::piped())
            .arg("--single-transaction")
            .arg("--quick")
            .arg("--lock-tables=false")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--databases")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--result-file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysqldump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysqldump_database(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("MYSQL_ROOT_PASSWORD");
/// C.mysql_restore_database(
/// 	C.CString("localhost"),
/// 	C.CString("3306"),
/// 	C.CString("root"),
/// 	C.CString("mysql"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn mysql_restore_database(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    database: *const libc::c_char,
    filename: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_database = unsafe { CStr::from_ptr(database).to_bytes() };
    let buf_filename = unsafe { CStr::from_ptr(filename).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("mysql")
            .stdout(std::process::Stdio::piped())
            .arg("-vvv")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--database")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--execute=source "#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysql_restore_database(): {}", output.status);
        //assert!(output.status.success());
    } else {
        let child = Command::new("mysql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--database")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--execute=source "#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysql_restore_database(): {}", output.status);
        //assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("MYSQL_ROOT_PASSWORD");
/// C.mysql_restore_all_databases(
/// 	C.CString("localhost"),
/// 	C.CString("3306"),
/// 	C.CString("root"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn mysql_restore_all_databases(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    filename: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_filename = unsafe { CStr::from_ptr(filename).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("mysql")
            .stdout(std::process::Stdio::piped())
            .arg("-v")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--execute=source "#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysql_restore_all_databases(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("mysql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--user")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--execute=source "#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("mysql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("mysql_restore_all_databases(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("PGPASSWORD");
/// C.pg_dump_database(
/// 	C.CString("localhost"),
/// 	C.CString("5432"),
/// 	C.CString("postgres"),
/// 	C.CString("postgres"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn pg_dump_database(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    database: *const libc::c_char,
    filename_pattern: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_database = unsafe { CStr::from_ptr(database).to_bytes() };
    let buf_filename_pattern = unsafe { CStr::from_ptr(filename_pattern).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("pg_dump")
            .stdout(std::process::Stdio::piped())
            .arg("--verbose")
            .arg("--encoding=utf8")
            .arg("--format=plain")
            .arg("--jobs=1")
            .arg("--no-owner")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("pg_dump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("pg_dump_database(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("pg_dump")
            .stdout(std::process::Stdio::piped())
            .arg("--encoding=utf8")
            .arg("--format=plain")
            .arg("--jobs=1")
            .arg("--no-owner")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("pg_dump command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("pg_dump_database(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("PGPASSWORD");
/// C.pg_dump_all_databases(
/// 	C.CString("localhost"),
/// 	C.CString("5432"),
/// 	C.CString("postgres"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn pg_dump_all_databases(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    database: *const libc::c_char,
    filename_pattern: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_database = unsafe { CStr::from_ptr(database).to_bytes() };
    let buf_filename_pattern = unsafe { CStr::from_ptr(filename_pattern).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("pg_dumpall")
            .stdout(std::process::Stdio::piped())
            .arg("--verbose")
            .arg("--encoding=utf8")
            .arg("--format=plain")
            .arg("--jobs=1")
            .arg("--no-owner")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("pg_dumpall command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("pg_dump_all_databases(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("pg_dumpall")
            .stdout(std::process::Stdio::piped())
            .arg("--encoding=utf8")
            .arg("--format=plain")
            .arg("--jobs=1")
            .arg("--no-owner")
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename_pattern.to_vec()).unwrap(),
            )
            .spawn()
            .expect("pg_dumpall command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("pg_dump_all_databases(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("PGPASSWORD");
/// C.psql_restore_database(
/// 	C.CString("localhost"),
/// 	C.CString("5432"),
/// 	C.CString("postgres"),
/// 	C.CString("postgres"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn psql_restore_database(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    database: *const libc::c_char,
    filename: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_database = unsafe { CStr::from_ptr(database).to_bytes() };
    let buf_filename = unsafe { CStr::from_ptr(filename).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("psql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("psql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("psql_restore_database(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("psql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg("--dbname")
            .arg(String::from_utf8(buf_database.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("psql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("psql_restore_database(): {}", output.status);
        assert!(output.status.success());
    }
}

/// Example
/// ```no_run
/// // Golang
/// os.Getenv("PGPASSWORD");
/// C.psql_restore_all_databases(
/// 	C.CString("localhost"),
/// 	C.CString("5432"),
/// 	C.CString("postgres"),
/// 	C.CString("dump.sql"),
/// 	C.CString("true"),
/// );
/// ```
#[no_mangle]
pub extern "C" fn psql_restore_all_databases(
    host: *const libc::c_char,
    port: *const libc::c_char,
    user: *const libc::c_char,
    filename: *const libc::c_char,
    verbose: *const libc::c_char,
) {
    let buf_host = unsafe { CStr::from_ptr(host).to_bytes() };
    let buf_port = unsafe { CStr::from_ptr(port).to_bytes() };
    let buf_user = unsafe { CStr::from_ptr(user).to_bytes() };
    let buf_filename = unsafe { CStr::from_ptr(filename).to_bytes() };

    if unsafe { CStr::from_ptr(verbose).to_str() } == Ok("true") {
        let child = Command::new("psql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("psql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("psql_restore_all_databases(): {}", output.status);
        assert!(output.status.success());
    } else {
        let child = Command::new("psql")
            .stdout(std::process::Stdio::piped())
            .arg("--host")
            .arg(String::from_utf8(buf_host.to_vec()).unwrap())
            .arg("--port")
            .arg(String::from_utf8(buf_port.to_vec()).unwrap())
            .arg("--username")
            .arg(String::from_utf8(buf_user.to_vec()).unwrap())
            .arg(
                r#"--file="#.to_owned()
                    + &String::from_utf8(buf_filename.to_vec()).unwrap(),
            )
            .spawn()
            .expect("psql command failed to start");

        let output = child
            .wait_with_output()
            .expect("failed to wait display on child");

        println!("psql_restore_all_databases(): {}", output.status);
        assert!(output.status.success());
    }
}

// struct Runner {
//     waker: task::Waker,
//     timeout: time::Duration,
//     max_workers: u64,
// }

// #[cfg(test)]
// pub mod test {
//
//     use super::*;
//
//     // This is meant to do the same stuff as the main function in the .go files.
//     #[test]
//     fn simulated_main_function() {
//
//     }
// }
