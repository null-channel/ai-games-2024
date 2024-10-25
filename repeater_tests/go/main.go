package main

import (
	"database/sql"
	"log"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

var schema = `
CREATE TABLE person (
    first_name text,
    last_name text,
    email text
);

CREATE TABLE place (
    country text,
    city text NULL,
    telcode integer
)`

type Person struct {
	FirstName string `db:"first_name"`
	LastName  string `db:"last_name"`
	Email     string
}

type Place struct {
	Country string
	City    sql.NullString
	TelCode int
}

func InsertPerson(tx *sqlx.Tx, p Person) error {
	_, err := tx.NamedExec(`INSERT INTO person (first_name, last_name, email) VALUES (:first_name, :last_name, :email)`, p)
	return err
}

func GetPerson(tx *sqlx.Tx, firstName string) (Person, error) {
	var p Person
	err := tx.Get(&p, "SELECT * FROM person WHERE first_name=$1", firstName)
	return p, err
}

func main() {
	// this Pings the database trying to connect
	// use sqlx.Open() for sql.Open() semantics
	db, err := sqlx.Connect("postgres", "user=foo dbname=bar sslmode=disable")
	if err != nil {
		log.Fatalln(err)
	}

	// exec the schema or fail; multi-statement Exec behavior varies between
	// database drivers;  pq will exec them all, sqlite3 won't, ymmv
	db.MustExec(schema)

	tx := db.MustBegin()

	jasonP := Person{FirstName: "Jason", LastName: "Moiron", Email: "jmoiron@jmoiron.net"}
	InsertPerson(tx, jasonP)
	johnP := Person{FirstName: "John", LastName: "Doe", Email: "johndoe@gmail.com"}
	InsertPerson(tx, johnP)

	us := Place{Country: "United States", City: sql.NullString{"New York", true}, TelCode: 1}
	hk := Place{Country: "Hong Kong", City: sql.NullString{"", false}, TelCode: 852}
	sg := Place{Country: "Singapore", City: sql.NullString{"", false}, TelCode: 65}

	// insert into places table

	// Named queries can use structs, so if you have an existing struct (i.e. person := &Person{}) that you have populated, you can pass it in as &person
	tx.Commit()

	// Query the database, storing results in a []Person (wrapped in []interface{})
	people := []Person{}
	db.Select(&people, "SELECT * FROM person ORDER BY first_name ASC")
	jason, john := people[0], people[1]

	// Select US by name

	// Select Mr. Smith from the database

	// batch insert

	personStructs := []Person{
		{FirstName: "Ardie", LastName: "Savea", Email: "asavea@ab.co.nz"},
		{FirstName: "Sonny Bill", LastName: "Williams", Email: "sbw@ab.co.nz"},
		{FirstName: "Ngani", LastName: "Laumape", Email: "nlaumape@ab.co.nz"},
	}

	// batch insert with maps

}
