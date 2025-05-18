# Toast

![Logo](bucket_data/thumbnails/d410d185-f372-43e4-bc4b-888bada43d83)

A blogging application to write about my hobby projects.

## Set-Up for Local Development or Debugging

### Install tools

```shell
cargo install sqlx-cli --no-default-features --features rustls,sqlite
cargo install 
```

### Run

```shell
# Set up environment and generated files
just down up gen
# Run backend
just run
# Run frontend
just dev
```

## Navigation

* The website itself can be found at http://localhost:8000
* MinIO, which hosts all images and markdown files, can be accessed at http://localhost:8001

## Useful commands

### Create a database dump

Useful for creating fixtures. Run in the database container:

```shell
pg_dump \
	-d toast \
	-U user \
	--data-only \
	--inserts
```

## License

Copyright (C) 2025 Rick van der Wal

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
