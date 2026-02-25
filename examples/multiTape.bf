s"Hello world from tape 0 (default)"
T1 										// Switch to tape '1'
s"Hello world from tape 1"
Ta 										// Switch to tape 'a'
s"Hello world from tape a"
T_ 										// Switch to tape '_'
s"Hello world from tape _"

T0 w(./tape0.txt)						// Switch to tape '0' and save tape to file
T1 w(./tape1.txt)						// Switch to tape '1' and save tape to file
Ta w(./tapea.txt)						// Switch to tape 'a' and save tape to file
T_ w(./tape_.txt)						// Switch to tape '_' and save tape to file