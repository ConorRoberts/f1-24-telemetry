#!/bin/bash

diesel migration generate --diff-schema=./src/db/schema.rs $1