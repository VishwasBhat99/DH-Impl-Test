#!/usr/bin/env python
import collections
import csv
import sys
import argparse

arg_parser = argparse.ArgumentParser()
arg_parser.add_argument("input_file", help = "Path to input file")
arg_parser.add_argument("--csv_file", help  = "Optional parameter to generate a csv", action = "store_true")
args = arg_parser.parse_args()

input_file = open(args.input_file, "r")
# Divide the total duration in nano seconds by 10^9 to convert the total duration into seconds.
ns_to_sec = pow(10,9)
summary = {}

for line_no,line in enumerate(input_file):
    diag_cmpnts = line.split()
    try:
        action = diag_cmpnts[3].strip(',')
    except:
        print "Unexpected log format at line no: " + str(line_no+1)
        continue
    try:
        existing_duration = float(summary.get(action, 0))
    except:
        print "Cannot convert existing action value string to float at line no: " + str(line_no+1) + '\n'
        print "line no: " + str(line_no+1) + " -> " + line + '\n'
        continue
    try:
        action_duration_index = len(diag_cmpnts)-1
        action_duration = (float(diag_cmpnts[action_duration_index]) + existing_duration)
    except:
        print "Cannot convert new action value string to float at line no: " + str(line_no+1) + '\n'
        print "line no: " + str(line_no+1) + " -> " + line + '\n'
        continue
    summary[action] = action_duration
input_file.close()
print "Action: Duration(secs)"
for item in summary.items():
    res = str(item[0]) + " : " + str(item[1] / ns_to_sec) + '\n'
    sys.stdout.write(str(res))
if args.csv_file:
    op_file = open('diag_summary.csv','w')
    with op_file:
        writer = csv.writer(op_file)
        writer.writerow(['Action','Duration(secs)'])
        writer.writerows(summary.items())
