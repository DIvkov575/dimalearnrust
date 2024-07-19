#!/bin/bash

dirs=($(ls))
for file in ${dirs[@]}; do
	new_file=${file/-experiments/}
	mv $file $new_file
done

