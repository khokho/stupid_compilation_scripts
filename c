#!/bin/bash
in=./in.in
ex=./ex
cg=./x.cpp

if [ -n "$1" ] 
then
   in=$1
   echo input is now: $in
fi

if [ -n "$2" ] 
then
   ex=$2
   echo executable is now: $ex
fi

if [ "$(<md)" != "$(echo $(md5sum $cg))" ]
then
   echo compiling...
   g++ $cg -g -o $ex |& egrep --color 'error|in expansion'
   x=${PIPESTATUS[0]}
   if [ $x -eq 0 ]
   then
      echo $(md5sum $cg)> md
   fi
else
   x=0
fi



if [ $x -eq 0 ]
then
   echo "**********************"
   echo 
   cat $in
   echo 
   echo ----------------------
   /usr/bin/time -f "\nTime: %e" $ex < $in
   rv=$?
   if (( rv != 0 )) 
   then
      ./d
   fi
else 
   echo CE!
fi

