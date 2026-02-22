s"Hello World!"
w(./tapeWriteTest.txt)  // Writes current tape to file (startng from cell with address 0)
<[^<]>                  // Clears the tape
r(./tapeWriteTest.txt)  // Reads the file to tape (starting from current pointer position)
<[<]
>[.>]
a(./tapeWriteTest.txt)  // Appends current tape to file (startng from cell with address 0)
w(@./CWDRelativeFile.txt)