#!/bin/bash
VALID=0
INVALID=0
while read line; do
  
  MIN=$((`echo $line | cut -f1 -d'-'` - 1))
  MAX=$((`echo $line | cut -f2 -d'-'| cut -f1 -d' '` - 1))
  LETTER=`echo $line | cut -f1 -d':'| cut -f2 -d' '`
  PW=`echo $line | cut -f2 -d':'| cut -f2 -d' '`
  FIRST=${PW:$MIN:1}
  SECOND=${PW:$MAX:1}
  
  #echo $FIRST
  #echo $SECOND
  #echo "$MIN-$MAX $LETTER $PW"

  if [ "$SECOND" != "$FIRST" ] && ( [ "$LETTER" == "$FIRST" ] || [ "$LETTER" == "$SECOND" ] )
  then
    VALID=$(($VALID+1))
  else
    INVALID=$(($INVALID+1))
  fi
done <puzzle.txt
echo "Valid $VALID"
echo "Invalid $INVALID"
