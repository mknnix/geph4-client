#!/bin/sh

txt2json(){
	if test -z "$1"
	then
		echo "Internal Error: missing args"
		exit 1
	fi

	cat "$1" | awk 'BEGIN{printf "["} {printf "\""$0"\","} END {print "\"\"]"}'
}

racket generate-china.sh > china-domains.txt
txt2json china-domains.txt > china-domains.json

curl https://raw.githubusercontent.com/17mon/china_ip_list/master/china_ip_list.txt > china-ips.txt
txt2json china-ips.txt > china-ips.json

