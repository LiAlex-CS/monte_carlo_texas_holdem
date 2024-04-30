#!/bin/bash

if [ ! -d "./output_backups" ]; then
    mkdir ./output_backups
fi

cp *.csv ./output_backups