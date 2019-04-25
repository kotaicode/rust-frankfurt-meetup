USER=$(shell whoami)
APP_NAME ?= myApp

new:
	docker run -e USER=$(USER) -it -v "$(shell pwd):/usr/src" -w /usr/src --rm rust:latest cargo new --bin $(APP_NAME)

test:
	docker run -it -v "$(shell pwd)/$(APP_NAME):/usr/src/app" -w /usr/src/app --rm rust:latest cargo test

run:
	docker run -it -v "$(shell pwd)/$(APP_NAME):/usr/src/app" -w /usr/src/app --rm rust:latest cargo run


