from PIL import Image
import sys

if len(sys.argv) != 3:
  print "Usage: python convert.py <input_file> <output_file_name>"
  sys.exit(0)

img = Image.open(str(sys.argv[1]))
pix = img.load()

grsc = list(reversed("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,\"^`'. "))

f = open(sys.argv[2]+".txt", "w")
matrix = []
for i in range(img.size[1]):
  row = []
  for j in range(img.size[0]):
    row.append(grsc[ pix[j,i][0] / len(grsc) ])
    f.write(grsc[ pix[j,i][0] / len(grsc) ])
  matrix.append(row)
  row = []
  f.write('\n')

print "Finished"
