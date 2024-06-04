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

rm -f results.txt

./secure_classifier

./secure_classifier

DIFF_RESULT=$(echo "[chris:dog.jpg:2]
12.50%: miniature schnauzer
12.40%: malamute
[kyle:eagle.jpg:3]
87.09%: bald eagle
11.46%: kite
 0.53%: hen
[chris:dog.jpg:2]
12.50%: miniature schnauzer
12.40%: malamute
[kyle:eagle.jpg:3]
87.09%: bald eagle
11.46%: kite
 0.53%: hen
" | diff results.txt -)

echo -e $DIFF_RESULT