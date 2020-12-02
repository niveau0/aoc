#!/bin/bash
VALID=0
INVALID=0
while read line; do
  
  MIN=$((`echo $line | cut -f1 -d'-'`))
  MAX=$((`echo $line | cut -f2 -d'-'| cut -f1 -d' '`))
  LETTER=`echo $line | cut -f1 -d':'| cut -f2 -d' '`
  PW=`echo $line | cut -f2 -d':'| cut -f2 -d' '`
  CHARCOUNT=$((`echo $PW | grep -o $LETTER | wc -l`))
  
  if (( $CHARCOUNT < $MIN )) || (( $CHARCOUNT > $MAX ))
  then
    #echo "Bad: $MIN-$MAX $LETTER $PW => count: $CHARCOUNT"
    INVALID=$(($INVALID+1))
  else
    VALID=$(($VALID+1))
  fi
done <puzzle.txt
echo "Valid $VALID"
echo "Invalid $INVALID"
