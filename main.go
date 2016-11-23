package main

import (
	"flag"
	"fmt"
	"github.com/golang/glog"
	"net/http"
	"os"

	"github.com/rubenv/sql-migrate"
	"gopkg.in/macaron.v1"

	"database/sql"
	_ "github.com/go-sql-driver/mysql"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
)

func usage() {
	fmt.Fprintf(os.Stderr, "usage: example -stderrthreshold=[INFO|WARN|FATAL] -log_dir=[string]\n")
	flag.PrintDefaults()
	os.Exit(2)
}

func init() {
	flag.Usage = usage
	// NOTE: This next line is key you have to call flag.Parse() for the command line
	// options or "flags" that are defined in the glog module to be picked up.
	flag.Parse()
}

func main() {
	//db, err := sql.Open("mysql", "user:password@tcp(127.0.0.1:3306)/hello")
	db, err := sql.Open("sqlite3", "tech-db-homework.db")
	if err != nil {
		glog.Fatal(err)
	}

	migrations := &migrate.MemoryMigrationSource{
		Migrations: []*migrate.Migration{
			&migrate.Migration{
				Id:   "123",
				Up:   []string{"CREATE TABLE people (id INT)"},
				Down: []string{"DROP TABLE people"},
			},
		},
	}

	n, err := migrate.Exec(db, "sqlite3", migrations, migrate.Up)
	if err != nil {
		glog.Fatal(err)
	}
	fmt.Printf("Applied %d migrations!\n", n)

	err = db.Ping()
	if err != nil {
		glog.Warning(err)
	}

	defer db.Close()

	m := macaron.Classic()
	m.Get("/", myHandler)

	glog.Info("Server is running...")
	glog.Info(http.ListenAndServe("0.0.0.0:4000", m))
}

func myHandler(ctx *macaron.Context) string {
	return "the request path is: " + ctx.Req.RequestURI
}
