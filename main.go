package main

import (
	"flag"
	"fmt"
	"github.com/golang/glog"
	"net/http"
	"os"

	"gopkg.in/macaron.v1"

	"database/sql"
	_ "github.com/go-sql-driver/mysql"
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
	db, err := sql.Open("mysql", "user:password@tcp(127.0.0.1:3306)/hello")
	if err != nil {
		glog.Fatal(err)
	}

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
