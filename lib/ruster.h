void mysqldump_database(char *host, char *port, char *user, char *database, char *filename_pattern, char *verbose);
void pg_dump_database(char *host, char *port, char *user, char *database, char *filename_pattern, char *verbose);
void mysqldump_all_databases(char *host, char *port, char *user, char *filename, char *verbose);
void pg_dump_all_databases(char *host, char *port, char *user, char *filename, char *verbose);
void mysql_restore_database(char *host, char *port, char *user, char *database, char *filename, char *verbose);
void mysql_restore_all_databases(char *host, char *port, char *user, char *filename, char *verbose);
void psql_restore_database(char *host, char *port, char *user, char *database, char *filename, char *verbose);
void psql_restore_all_databases(char *host, char *port, char *user, char *filename, char *verbose);