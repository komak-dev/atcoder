def ngram(s, n=2):
    return [s[i:i+n] for i in range(len(s)-n+1)]

x = "paraparaparadise"
y = "paragraph"
X = set(ngram(x, 3))
Y = set(ngram(y, 3))
print(len(X & Y) / len(X | Y))

print(X)
print(Y)
