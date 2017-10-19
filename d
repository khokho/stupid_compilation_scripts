#!/bin/bash
gdb -q -ex "r < in.in" -ex "bt" -ex "q" ex | sed '1,/Starting program/d'
