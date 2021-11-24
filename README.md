# ruster


![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)
![Continuous integration](https://github.com/actions-rs/cargo/workflows/Continuous%20integration/badge.svg)
<!-- [![CodeQL](https://github.com/e-monkeys-tech/dbutils_shlib_rs/workflows/CodeQL/badge.svg)](https://github.com/e-monkeys-tech/dbutils_shlib_rs/actions?query=workflow%3Arust) -->

ruster is a library using **ffi** for database management with **psql/pg_dump + mysql/mysqldump** written in Rust.

The generated C static and shared libraries can be reused in other languages (Golang from example).

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

```go
/*                                             
#cgo LDFLAGS: -L./lib -lruster                 
#include "./lib/ruster.h"                      
*/                                             
import "C"                                     

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
