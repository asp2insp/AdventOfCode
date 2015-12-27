#!/usr/bin/python
s = raw_input('> ')
bs = s.split(',')
nums = map(lambda b: int(b), bs)
print '"{}"'.format(''.join(map(lambda n: chr(n), nums)))
