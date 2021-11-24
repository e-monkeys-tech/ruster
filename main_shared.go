package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lruster
#include "./lib/ruster.h"
*/
import "C"

import (
     "os"
)

func main() {
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