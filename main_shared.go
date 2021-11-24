package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lruster
#include "./lib/ruster.h"
*/
import "C"

import (
//     "fmt"
//     "os/exec"
     "os"
    //"time"
)

func main() {
// 	cmd := exec.Command("ls", "-lah")
// 	cmd.Stdout = os.Stdout
// 	cmd.Stderr = os.Stderr
// 	err := cmd.Run()
// 	if err != nil {
// 		fmt.Println(err)
// 	}
//  os.Getenv("MYSQL_PWD")
//  os.Getenv("MYSQL_ROOT_PASSWORD")
// 	C.mysqldump_database(
// 		C.CString("localhost"),
// 		C.CString("3306"),
// 		C.CString("root"),
// 		C.CString("mysql"),
// 		C.CString("dump.sql"),
// 		C.CString("true"),
// 	);
//    time.Sleep(time.Duration(250)*time.Millisecond)
//    C.mysql_restore_database(
//     	C.CString("localhost"),
//     	C.CString("3306"),
//     	C.CString("root"),
//     	C.CString("mysql"),
//     	C.CString("dump.sql"),
//     	C.CString("true"),
//     );
    os.Getenv("PGPASSWORD")
    err := func() error {
            C.pg_dump_database(
    		C.CString("localhost"),
    		C.CString("5432"),
    		C.CString("postgres"),
    		C.CString("postgres"),
    		C.CString("pg_dump.sql"),
    		C.CString("true"),
    		);
    		return nil
    }
    if err() == nil {
        func() {
            C.psql_restore_database(
        	C.CString("localhost"),
        	C.CString("5432"),
        	C.CString("postgres"),
        	C.CString("postgres"),
        	C.CString("pg_dump.sql"),
        	C.CString("true"),
            );
        }()
    }
}