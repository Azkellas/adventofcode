from sys import stdin
import re

vowelReg = re.compile("[aeiou]")
def enough_vowels(word):
    print(len(vowelReg.findall(word)), end=" ")
    return len(vowelReg.findall(word)) >= 3

banWords = re.compile("ab|cd|pq|xy")
def no_ban_words(word):
    print(len(banWords.findall(word)), end=" ")
    return len(banWords.findall(word)) == 0

doubleLetter = re.compile(r"(\w)\1")
def double_letter(word):
    print(len(doubleLetter.findall(word)))
    return len(doubleLetter.findall(word)) > 0

def valid(word):
    return enough_vowels(word) and no_ban_words(word) and double_letter(word)

total = 0
for line in stdin:
    total += valid(line)
    print(line)
print(total)
