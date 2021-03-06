# ruster

[![docs](https://img.shields.io/docsrs/libruster/1.3.1)](https://docs.rs/libruster/1.3.1/libruster/)
[![Rust Reference](https://img.shields.io/badge/docs.rs-rustdoc-green)](https://docs.rs/libruster/1.3.1/libruster/)
[![Coverage Status](https://codecov.io/github.com/e-monkeys-tech/ruster/branch/master/graph/badge.svg)]()
[![crates.io](https://img.shields.io/badge/crates.io-v1.3.1-orange)](https://crates.io/crates/libruster/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)]()
[![Continuous integration](https://github.com/actions-rs/cargo/workflows/Continuous%20integration/badge.svg)]()
<!-- https://app.travis-ci.com/github/e-monkeys-tech/ruster -->

Ruster is a library using **ffi** for database management with **psql/pg_dump + mysql/mysqldump** written in Rust.

The generated C static and shared libraries can be reused in other languages (Golang for example).

## Build ruster

The build of project produces 2 go binaries, 1 static library (_libruster.a_) and 1 shared library (_libruster.so_).

The headers of C functions are in _lib/ruster.h_ and must be present before code generation.

```bash
make build-shared
```

```bash
make build-static
```

## Using ruster

### PostgreSQL

```go
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
```

### MySQL - Mariadb

```go
/*
#cgo LDFLAGS: -L./lib -lruster
#include "./lib/ruster.h"
*/
import "C"

import (
     "os"
)

func main() {
  os.Getenv("MYSQL_PWD")
  err := func() error {
    C.mysqldump_database(
      C.CString("localhost"),
      C.CString("3306"),
      C.CString("root"),
      C.CString("mysql"),
      C.CString("dump.sql"),
      C.CString("true"),
    );
      return nil
  }
  time.Sleep(time.Duration(250)*time.Millisecond)
  if err() == nil {
    func() {
      C.mysql_restore_database(
        C.CString("localhost"),
        C.CString("3306"),
        C.CString("root"),
        C.CString("mysql"),
        C.CString("dump.sql"),
        C.CString("true"),
      );
    }()
  }
}
```

## Run ruster examples

```bash
make run-shared
```

```bash
make run-static
```

## Bench

```bash
./time main_shared
./time main_static
```

## Tests

```bash
./time test-rust-lib
```
