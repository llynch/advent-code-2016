import md5
import sys

input_ = sys.argv[1]

count = 0
nbdigits = 0
digits = []
while True:
    m = md5.new()
    m.update("{}{}".format(input_, count))
    result = m.hexdigest()
    count += 1
    if result.startswith("00000"):
        digits.append(result[5])
        nbdigits += 1

    if nbdigits == 8:
        break;
print "".join(digits)

count = 0
nbdigits = 0
digits = [" "] * 8
while True:
    m = md5.new()
    m.update("{}{}".format(input_, count))
    result = m.hexdigest()
    count += 1
    if result.startswith("00000"):
        position = result[5]
        digit = result[6];
        try:
            pos = int(position)
        except:
            continue
        if pos > 7:
            continue
        if digits[pos] == ' ':
            digits[pos] = digit
            nbdigits += 1

        if nbdigits == 8:
            break;

print "".join(digits)




