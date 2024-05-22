#!/bin/bash

echo "Check pre-condition"

make
rc=$?
if [ $rc -ne 0 ]
then
        echo "Error when running the make command"
        exit 1
fi
make
rc=$?
if [ $rc -ne 0 ]
then
        echo "Error when running the make command"
        exit 1
fi

if [ ! -e "./secure_classifier" ]
then
        echo "Error: Running make did not create the secure_classifier file"
        exit 1
fi

if [ ! -x "./secure_classifier" ]
then
        echo "Error: secure_classifier is not executable"
        exit 1
fi

export LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH
./secure_classifier requests.txt