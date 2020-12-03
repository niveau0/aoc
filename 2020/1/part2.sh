#!/bin/bash
DATA=(`cat puzzle.txt | sed  's/$/ /' | tr -d '\n'`);for i in ${!DATA[@]}; do for j in ${!DATA[@]}; do for k in ${!DATA[@]}; do s=$((${DATA[$i]}+${DATA[$j]}+${DATA[$k]})); if [ $s -eq "2020" ]; then echo $i $j $k ${DATA[$i]}+${DATA[$j]}+${DATA[$k]}=$s; break 3; fi;  done; done;  done

